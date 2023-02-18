use crate::error::{ Error, Result };
use crate::meta::version::Version;
use crate::meta::version::OptionType;
use crate::meta::version::PackVersionSpecifier;
use crate::runtime_meta::Warning;
use crate::util::RON;
use super::action::Action;
use super::{ ASSETS_DIR_NAME, META_NAME };
use tokio::fs;

#[derive(Debug)]
pub struct VersionRuntimeMeta {
	path: String,
	versions: Vec<PackVersionSpecifier>,
	processing_option: OptionType,
	actions: Vec<Action>,
	warnings: Vec<Warning>
}

impl VersionRuntimeMeta {
	pub async fn new(path: &str) -> Result<Self> {
		// if !fs::metadata(path).await?.is_dir() {
		// 	return Err
		// }
		let warnings = vec![];
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
					processing_option: r#type.unwrap_or(OptionType::CopyPaste)
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
					if !file.ends_with(".png") { continue }

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

			OptionType::Random { block_id, mirror, y } => {
				vec![]
			}
		};

		let new = Self {
			path: path.into(),
			versions,
			processing_option,
			actions,
			warnings
		};

		Ok(new)
	}
}
