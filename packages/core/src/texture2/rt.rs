use crate::option2;
use crate::util::fs;
use crate::util::path_builder3::WithTextureID;
use super::{ meta, nm, nr };
use super::error::*;

pub struct TextureRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	default: nr::Default,
	options: nr::Options
}

impl TextureRuntime {
	pub(crate) async fn new() -> Result<Option<Self>> {
		todo!()
	}

	pub(crate) async fn new2(p: &WithTextureID<'_>) -> Result<Option<Self>> {
		let dir = p.texture_dir_silent_fail().await?;
		let meta_path = p.texture_manifest_silent_fail().await?;
		let meta_file = fs::read_to_string2(meta_path).await?;
		let meta::TextureUnversioned {
			name,
			description,
			default
		} = meta::deserialise_texture(&meta_file)?;

		// let name = name.transmute_nom();
		// let description = description.transmute_nom();
		// let default = default
		// 	.map_nom_some(option2::nr::ID::new);

		let option_entries = p.option_entries_dir_checked().await?;
		let mut read_dir = fs::read_dir2(option_entries).await?;

		while let Some(file) = read_dir.next().await? {
			let option_id = file.file_name();
			let option_id = option_id.to_str()
				.ok_or_else(|| Error::NonUtf8Path)?;
			let option_id = option2::nr::ID::new(option_id.into());

			// TODO
			let option = option2::OptionRuntime::new().await?;

			// if let Some(option) = option {
			// 	// TODO something
			// }
		}

		// read option dir first,
		//
		// then check if the specified default is available
		// and act accordingly
		// (didn't actually do this default existence check in previous version)
		// i guess hard error for now? can make it nicer later
		//
		// keep this comment
		// TODO consider making nonexistent default a warning
		// orrrrr maybe even add like, a strictness option that controls this

		todo!()
	}
}
