use super::*;

pub struct Version(InnerVersion);

pub struct InnerVersion {
	pub file: file::Version,
	pub path: Utf8PathBuf,
	pub mc_version: &'static PackVersion
}

impl_deref!(*Version => InnerVersion);

impl Version {
	pub async fn new(path: Utf8PathBuf, mc_version: &'static PackVersion) -> Result<Option<Self>> {
		let file = read_manifest::<file::Version>(&path).await?;

		// TODO verify the version
		let version_matches = match file {
			file::Version::V1(ref file) => {
				// TODO currently other_versions is ignored
				file.versions.iter()
					.any(|v| v.contains_mc_version(mc_version))
			}
		};

		Ok(if version_matches {
			Some(Version(InnerVersion { file, path, mc_version }))
		} else {
			None
		})
	}
}
