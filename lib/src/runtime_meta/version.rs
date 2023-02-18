use ahash::{ HashMapExt, HashSetExt, RandomState };
use crate::error::{ Error, Result };
use crate::mc_structs::blockstate::{ Blockstate, BlockstateEntry };
use crate::mc_structs::model::Model;
use crate::meta::version::Version;
use crate::meta::version::OptionType;
use crate::meta::version::PackVersionSpecifier;
use crate::runtime_meta::Warning;
use crate::util::hash;
use crate::util::RON;
use std::collections::HashMap;
use std::collections::HashSet;
use super::action::Action;
use super::{ ASSETS_DIR_NAME, META_NAME };
use tokio::fs;

#[derive(Debug)]
pub struct VersionRuntimeMeta {
	pub path: String,
	pub shortpath: String,
	pub versions: Vec<PackVersionSpecifier>,
	pub processing_option: OptionType,
	pub actions: Vec<Action>,
	pub warnings: Vec<Warning>
}

impl VersionRuntimeMeta {
	pub async fn new(path: &str) -> Result<Self> {
		// if !fs::metadata(path).await?.is_dir() {
		// 	return Err
		// }
		let mut warnings = vec![];
		let manifest_path = format!("{path}/{META_NAME}");

		let manifest_file_meta = fs::metadata(&manifest_path).await
			.map_err(|e| Error::FileDoesNotExist { path: manifest_path.clone(), source: e })?;
		if !manifest_file_meta.is_dir() {}

		let file = fs::read_to_string(&manifest_path).await
			.map_err(|e| Error::IOError { source: e })?;
		let version = RON.from_str::<Version>(&file)
			.map_err(|e| Error::ParseErrorRonSpannedError {
				path: manifest_path,
				source: e
			})?;

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

		let assets_path = format!("{path}/{ASSETS_DIR_NAME}");
		let assets_metadata = fs::metadata(&assets_path).await
			.map_err(|e| Error::FileDoesNotExist { path: assets_path.clone(), source: e })?;
		if !assets_metadata.is_dir() { return Err(Error::AssetsPathIsNotDir { path: assets_path }) }

		let actions = match &processing_option {
			OptionType::CopyPaste => {
				let mut actions = vec![];

				let assets_contents = dbg!(crate::util::walk_dir(&assets_path).await?);
				for file in assets_contents {
					if !file.ends_with(".png") {
						warnings.push(Warning {
							message: format!("File does not appear to be a PNG image (file extension not `.png`): {file}")
						});
						continue
					}

					let mut relative_path = &file[path.len()..];
					dbg!(relative_path);
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

				let assets_contents = crate::util::walk_dir(&assets_path).await?;

				let mut model_and_blockstate = vec![];

				for file in assets_contents {
					if !file.ends_with(".png") {
						warnings.push(Warning {
							message: format!("File does not appear to be a PNG image (file extension not `.png`): {file}")
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
				let mut variants = vec![];
				let mut texture_src_paths = vec![];
				let mut texture_dedup_set: HashSet<String, RandomState> = HashSetExt::new();

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
						data: dbg!(serde_json::to_string(&model).unwrap()).into(),
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
					data: dbg!(serde_json::to_string(&blockstate).unwrap()).into(),
					src_files: texture_src_paths
				});

				actions
			}
		};

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		let new = Self {
			path: path.into(),
			shortpath,
			versions,
			processing_option,
			actions,
			warnings
		};

		Ok(new)
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
