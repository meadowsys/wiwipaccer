use crate::error::{ Error, Result };
use crate::meta::option::TextureOption;
use crate::runtime_meta::Warning;
use crate::runtime_meta::version::VersionRuntimeMeta;
use crate::util::RON;
use super::{ ASSETS_DIR_NAME, META_NAME };
use tokio::fs;

#[derive(Debug)]
pub struct OptionRuntimeMeta {
	path: String,
	name: String,
	description: String,
	versions: Vec<VersionRuntimeMeta>,
	warnings: Vec<Warning>
}

impl OptionRuntimeMeta {
	pub async fn new(path: &str) -> Result<Self> {
		let mut warnings = vec![];
		let manifest_path = format!("{path}/{META_NAME}");

		let _manifest_file_meta = fs::metadata(&manifest_path).await
			.map_err(|e| Error::FileDoesNotExist { path: manifest_path.clone(), source: e })?;

		let file = fs::read_to_string(&manifest_path).await
			.map_err(|e| Error::IOError { source: e })?;
		let option = RON.from_str::<TextureOption>(&file)
			.map_err(|e| Error::ParseErrorRonSpannedError {
				path: manifest_path,
				source: e
			})?;

		struct Destructure {
			name: String,
			description: String
		}

		let Destructure { name, description } = match option {
			TextureOption::V1 { name, description } => {
				Destructure {
					name,
					description: description.unwrap_or_else(|| "description not provided".into())
				}
			}
		};

		let mut versions = vec![];

		let mut dir_contents = fs::read_dir(&path).await
			.map_err(|e| Error::IOError { source: e })?;
		while let Some(dir_entry) = dir_contents.next_entry().await.map_err(|e| Error::IOError { source: e })? {
			let dir_entry_path = dir_entry.path();
			let dir_entry_path = dir_entry_path.to_str()
				.expect("invalid unicode paths unsupported");

			if dir_entry_path.ends_with(META_NAME) { continue }

			let dir_entry_metadata = fs::metadata(&dir_entry_path).await
				.map_err(|e| Error::IOError { source: e })?;
			if !dir_entry_metadata.is_dir() {
				warnings.push(Warning {
					message: format!("item in an option dir is not an option or the assets file: {dir_entry_path}")
				});
				continue
			}

			match VersionRuntimeMeta::new(dir_entry_path).await {
				Ok(version) => { versions.push(version) }
				Err(err) => { warnings.push(err.into_warning()) }
			}
		}

		Ok(OptionRuntimeMeta {
			path: path.into(),
			name,
			description,
			versions,
			warnings
		})
	}
}
