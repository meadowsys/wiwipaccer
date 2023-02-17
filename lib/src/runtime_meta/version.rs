use anyhow::Result;
use crate::util::RON;
use crate::meta::version::Version;
use crate::meta::version::OptionType;
use crate::meta::version::PackVersionSpecifier;
use std::fs;
use super::META_NAME;

#[derive(Debug)]
pub struct VersionRuntimeMeta {
	path: String,
	versions: Vec<PackVersionSpecifier>,
	processing_option: OptionType

}

impl VersionRuntimeMeta {
	pub fn new(path: &str) -> Result<Self> {

		let manifest_path = format!("{path}/{META_NAME}");

		let _manifest_file_meta = fs::metadata(&manifest_path)?;

		let file = fs::read_to_string(&manifest_path)?;
		let version = RON.from_str::<Version>(&file)?;

		let (versions, processing_option) = match version {
			Version::V1 { versions, r#type } => { (versions, r#type) }
		};

		Ok(Self {
			versions,
			processing_option: processing_option.unwrap_or(OptionType::CopyPaste),
			path: path.into()
		})
	}
}
