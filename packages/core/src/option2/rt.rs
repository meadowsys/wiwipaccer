use crate::util::fs;
use crate::util::path_builder3::WithOptionID;
use crate::version2::{ self, VersionRuntime };
use super::error::*;
use super::{ meta, nr };

pub struct OptionRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	versions: nr::Versions
}

impl OptionRuntime {
	pub(crate) async fn new(p: &WithOptionID<'_>) -> Result<Option<Self>> {
		let dir = p.option_dir_silent_fail().await?;
		let meta_path = p.option_manifest_silent_fail().await?;
		let meta_file = fs::read_to_string2(meta_path).await?;
		let meta::OptionUnversioned {
			name,
			description
		} = meta::deserialise_option(&meta_file)?;

		let name = name.transmute_nom();
		let description = description.transmute_nom();
		let id = nr::ID::new(p.option_id_ref().into());

		let versions = read_versions(p).await?;

		Ok(Some(Self {
			name,
			description,
			id,
			versions
		}))
	}
}

async fn read_versions(p: &WithOptionID<'_>) -> Result<nr::Versions> {
	let version_entries_dir = p.version_entries_dir_checked().await?;
	let mut versions_nom = nr::Versions::default();
	let versions = versions_nom.mut_inner();
	let mut read_dir = fs::read_dir2(version_entries_dir).await?;

	while let Some(file) = read_dir.next().await? {
		let file_name = file.file_name();
		let p = p.clone().with_version_id_osstr(&file_name)?;

		// TODO
		if let Some(v) = VersionRuntime::new().await? {
			let id = version2::nr::ID::new(p.version_id_ref().into());
			versions.insert(id, v);
		}
	}

	Ok(versions_nom)
}
