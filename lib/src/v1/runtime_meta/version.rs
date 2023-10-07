use ahash::{ HashMapExt, HashSetExt, RandomState };
use crate::error::{ Error, Result };
use super::super::mc_structs::blockstate::{ Blockstate, BlockstateEntry };
use super::super::mc_structs::model::Model;
use crate::v1::{ PackFormat, PackVersion, PACK_FORMATS };
use crate::meta::pack_version_specifier::PackVersionSpecifier;
use crate::meta::version::Version;
use crate::meta::version::OptionType;
use crate::runtime_meta::{ Message, MessageSeverity, read_meta_file };
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use super::super::util::hash;
use super::super::util::RON;
use super::super::util::walk_dir;
use std::collections::HashMap;
use std::collections::HashSet;
use super::action::Action;
use super::{ ASSETS_DIR_NAME, META_NAME };
use tokio::fs;

#[derive(Debug, serde::Serialize)]
pub struct WithoutMCVersion(InnerWithoutMCVersion);

#[derive(Debug, serde::Serialize)]
pub struct InnerWithoutMCVersion {
	pub path: String,
	pub shortpath: String,
	pub versions: Vec<PackVersionSpecifier>,
	pub processing_option: OptionType,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub enum WithMCVersion {
	Available(Available),
	Unavailable(Unavailable)
}

#[derive(Debug, serde::Serialize)]
pub struct Available(InnerAvailable);
#[derive(Debug, serde::Serialize)]
pub struct Unavailable(InnerUnavailable);

#[derive(Debug, serde::Serialize)]
pub struct InnerAvailable {
	pub path: String,
	pub shortpath: String,
	pub selected_version: String,
	pub versions: Vec<PackVersionSpecifier>,
	pub processing_option: OptionType,
	#[serde(skip_serializing)]
	pub actions: Vec<Action>,
	pub messages: Vec<Message>
}

#[derive(Debug, serde::Serialize)]
pub struct InnerUnavailable {
	pub path: String,
	pub shortpath: String,
	pub selected_version: String,
	pub versions: Vec<PackVersionSpecifier>,
	pub processing_option: OptionType,
	pub messages: Vec<Message>
}

crate::impl_deref!(WithoutMCVersion, target InnerWithoutMCVersion);
crate::impl_deref!(Available, target InnerAvailable);
crate::impl_deref!(Unavailable, target InnerUnavailable);

impl WithoutMCVersion {
	pub async fn new(path: &str) -> Result<Self> {
		let mut messages = vec![];
		let version = read_meta_file::<Version>(path).await?;

		struct Destructure {
			versions: Vec<PackVersionSpecifier>,
			processing_option: OptionType
		}

		let Destructure { versions, processing_option } = match version {
			Version::V1 { versions, r#type } => {
				Destructure {
					versions,
					processing_option: r#type.unwrap_or_else(|| OptionType::CopyPaste)
				}
			}
		};

		let versions = versions.into_iter()
			.filter(|mv| {
				let res = match mv {
					PackVersionSpecifier::PackVersion(mv) => {
						PACK_FORMATS.iter()
							.any(|rv| match rv.format {
								PackFormat::Verified(rv) | PackFormat::Unverified(rv) => { rv == *mv }
								PackFormat::Unknown | PackFormat::None => { false }
							})
					}
					PackVersionSpecifier::MCVersion(mv) => { PACK_FORMATS.iter().any(|rv| rv.name == mv) }
					PackVersionSpecifier::MCVersionRange(lowerv, upperv) => {
						let lowerv = PACK_FORMATS.iter().position(|rv| rv.name == lowerv);
						let upperv = PACK_FORMATS.iter().position(|rv| rv.name == upperv);
						lowerv.is_some() || upperv.is_some()
					}
				};

				if !res {
					match mv.to_mc_versions() {
						Ok(v) => {
							v.iter()
								.for_each(|v| messages.push(Error::MCVersionUnknown { version: v.clone() }.to_message()));
						}
						Err(e) => { messages.push(e.to_message()) }
					}
				}

				res
			})
			.collect::<Vec<_>>();

		let assets_path = format!("{path}/{ASSETS_DIR_NAME}");
		let assets_metadata = fs::metadata(&assets_path).await
			.map_err(|e| Error::FileDoesNotExist { path: assets_path.clone(), source: e })?;
		if !assets_metadata.is_dir() { return Err(Error::AssetsPathIsNotDir { path: assets_path }) }

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		Ok(Self(InnerWithoutMCVersion {
			path: path.into(),
			shortpath,
			versions,
			processing_option,
			messages
		}))
	}

	pub fn get_supported_mc_versions(&self) -> Result<Vec<PackVersion>> {
		// this is kinda inefficient, optimisation opportunity?
		// though to be fair this isn't hot code, so its not too important

		let mut versions = vec![];

		// for version in &self.versions {
		// 	version.contains(runtime_specifier)
		// }

		'outer: for version in PACK_FORMATS {
			for supported_version in &self.versions {
				if supported_version.contains(&PackVersionSpecifierRuntimeMeta::MCVersion(version.name.into()))? {
					versions.push(version);
					continue 'outer
				}
			}
		}

		Ok(versions.into_iter().cloned().collect())
	}
}

impl WithMCVersion {
	pub async fn from(
		version_without_mc_version: &WithoutMCVersion,
		mc_version: String
	) -> Result<Self> {
		let mut messages = version_without_mc_version.messages.clone();
		let path = &version_without_mc_version.path;
		let assets_path = format!("{path}/{ASSETS_DIR_NAME}");

		let actions = match &version_without_mc_version.processing_option {
			OptionType::CopyPaste => {
				let mut actions = vec![];

				let assets_contents = walk_dir(&assets_path).await?;
				for file in assets_contents {
					let mut relative_path = &file[path.len()..];
					if relative_path.starts_with('/') {
						// while next_char is being called, this will be one more than the index of
						// the character being read, so right before breaking the loop, we subtract
						// back that one. It's usize so cannot start with -1
						let mut slash_idx = 0;
						let mut chars = relative_path.chars();

						let mut next_char = || {
							slash_idx += 1;
							chars.next()
						};

						loop {
							if next_char() != Some('/') {
								slash_idx -= 1;
								break
							}
						}

						relative_path = &relative_path[slash_idx..];
					}
					actions.push(Action::CopyFile {
						from: file.clone(),
						to: relative_path.into()
					});
				}

				actions
			}

			OptionType::RandomCubeAll { block_id, mirror, y } => {
				let (block_ns, block_id) = block_id.split_once(':')
					.ok_or_else(|| Error::InvalidBlockID { id: block_id.clone() })?;


				let hash = hash(&format!("{block_ns}{block_id}{mirror:?}{y:?}"));

				let assets_contents = walk_dir(&assets_path).await?;

				let mut model_and_blockstate = vec![];

				for file in assets_contents {
					if !file.ends_with(".png") && !file.to_ascii_lowercase().ends_with(".png") {
						messages.push(Message {
							message: format!("File does not appear to be a PNG image (file extension not `.png`): {file}"),
							severity: MessageSeverity::Info
						});
						continue
					}

					let filename = std::path::Path::new(&file)
						.file_name()
						.unwrap()
						.to_str()
						.unwrap()
						.to_string();

					if let Some(y) = y {
						for y in y {
							let path = format!("block/{block_id}-{hash}/{filename}");

							if let Some(true) = mirror {
								model_and_blockstate.push(gen_model_and_blockstate(GenModelAndBlockstateParams {
									block_ns,
									file: file.clone(),
									parent: "block/cube_mirrored_all",
									path: path.clone(),
									y
								}));
							}

							model_and_blockstate.push(gen_model_and_blockstate(GenModelAndBlockstateParams {
								block_ns,
								file: file.clone(),
								parent: "block/cube_all",
								path,
								y
							}));

						}
					}
				}

				let mut actions = vec![];
				let mut variants = Vec::with_capacity(model_and_blockstate.len());
				let mut texture_src_paths = vec![];
				let mut texture_dedup_set = HashSet::<String, RandomState>::new();

				for mb in model_and_blockstate {
					let ModelAndBlockstate {
						blockstate_entry,
						model,
						model_file_path,
						texture_src_path,
						texture_dest_path
					} = mb;

					variants.push(blockstate_entry);

					actions.push(Action::WriteBytes {
						path: model_file_path,
						// this should never fail:
						// - Derive, no reason to fail
						// - HashMap contains string keys
						data: serde_json::to_string(&model).unwrap().into(),
						src_files: vec![texture_src_path.clone()]
					});

					if !texture_dedup_set.contains(&texture_src_path) {
						texture_dedup_set.insert(texture_src_path.clone());

						actions.push(Action::CopyFile {
							from: texture_src_path.clone(),
							to: texture_dest_path
						});
						texture_src_paths.push(texture_src_path);
					}
				}


				let mut blockstate = Blockstate {
					variants: HashMapExt::new()
				};
				blockstate.variants.insert("".into(), variants);

				actions.push(Action::WriteBytes {
					path: format!("assets/{block_ns}/blockstates/{block_id}.json"),
					data: serde_json::to_string(&blockstate).unwrap().into(),
					src_files: texture_src_paths
				});

				actions
			}
		};

		let supported = {
			let mc_version = PackVersionSpecifierRuntimeMeta::MCVersion(mc_version.clone());
			let iter = version_without_mc_version.versions.iter();
			let mut res = false;

			// iter through all of them because then we can find invalid
			for v in iter {
				if v.contains(&mc_version)? {
					res = true;
				}
			}
			res
		};

		if !supported {
			return Ok(Self::Unavailable(Unavailable(InnerUnavailable {
				path: path.into(),
				shortpath: version_without_mc_version.shortpath.clone(),
				selected_version: mc_version,
				versions: version_without_mc_version.versions.clone(),
				processing_option: version_without_mc_version.processing_option.clone(),
				messages
			})))
		}

		Ok(Self::Available(Available(InnerAvailable {
			path: path.into(),
			shortpath: version_without_mc_version.shortpath.clone(),
			selected_version: mc_version,
			versions: version_without_mc_version.versions.clone(),
			processing_option: version_without_mc_version.processing_option.clone(),
			actions,
			messages
		})))
	}
}

#[derive(Debug)]
struct ModelAndBlockstate {
	blockstate_entry: BlockstateEntry,
	model: Model,
	model_file_path: String,
	texture_src_path: String,
	texture_dest_path: String
}

struct GenModelAndBlockstateParams<'h> {
	block_ns: &'h str,
	file: String,
	parent: &'h str,
	path: String,
	y: &'h Option<u16>
}

fn gen_model_and_blockstate(params: GenModelAndBlockstateParams<'_>) -> ModelAndBlockstate {
	let GenModelAndBlockstateParams {
		block_ns,
		file,
		parent,
		path,
		y,
	} = params;

	let path_notdeviated = path;
	let hash = hash(&format!("{block_ns}{file}{parent}{path_notdeviated}{}", y.unwrap_or_else(|| 0)));
	let path = format!("{path_notdeviated}-{hash}");

	let mut textures: HashMap<String, String, RandomState> = HashMapExt::new();
	textures.insert("all".into(), format!("{block_ns}:{path_notdeviated}"));

	ModelAndBlockstate {
		blockstate_entry: BlockstateEntry {
			model: format!("{block_ns}:{path}"),
			y: *y
		},
		model: Model {
			parent: parent.into(),
			textures: Some(textures)
		},
		model_file_path: format!("assets/{block_ns}/models/{path}.json"),
		texture_src_path: file,
		texture_dest_path: format!("assets/{block_ns}/textures/{path_notdeviated}.png")
	}
}
