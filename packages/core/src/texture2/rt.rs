use ::mc_versions::MCVersionRef;
use crate::option2::{ self, OptionRuntime };
use crate::util::fs;
use crate::util::path_builder3::WithTextureID;
use super::{ meta, nm, nr };
use super::error::*;
use ::hashbrown::HashMap;
use ::serde::Serialize;

pub struct TextureRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	default: nr::Default,
	options: nr::Options
}

impl TextureRuntime {
	pub(crate) async fn new(p: &WithTextureID<'_>) -> Result<Option<Self>> {
		let dir = p.texture_dir_silent_fail().await?;
		let meta_path = p.texture_manifest_silent_fail().await?;
		let meta_file = fs::read_to_string2(meta_path).await?;
		let meta::TextureUnversioned {
			name,
			description,
			default
		} = meta::deserialise_texture(&meta_file)?;

		let name = name.transmute_nom();
		let description = description.transmute_nom();
		let id = nr::ID::new(p.texture_id_ref().into());
		let default = default
			.map_nom_some(option2::nr::ID::new);
		let options = read_options(p).await?;

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

		Ok(Some(Self {
			name,
			description,
			id,
			default,
			options
		}))
	}
}

async fn read_options(p: &WithTextureID<'_>) -> Result<nr::Options> {
	let option_entries_dir = p.option_entries_dir_checked().await?;
	let mut options_nom = nr::Options::default();
	let options = options_nom.mut_inner();
	let mut read_dir = fs::read_dir2(option_entries_dir).await?;

	while let Some(file) = read_dir.next().await? {
		let file_name = file.file_name();
		let p = p.clone().with_option_id_osstr(&file_name)?;

		// TODO
		if let Some(o) = OptionRuntime::new(&p).await? {
			let id = option2::nr::ID::new(p.option_id_ref().into());
			options.insert(id, o);
		}
	}

	Ok(options_nom)
}

#[derive(Serialize)]
pub struct FrontendData<'h> {
	name: &'h nr::Name,
	description: &'h nr::Description,
	id: &'h nr::ID,
	default: &'h nr::Default,
	options: HashMap<&'h str, option2::FrontendData<'h>>
}

impl<'h> FrontendData<'h> {
	pub fn new(texture: &'h TextureRuntime, mc_version: MCVersionRef) -> Self {
		let name = &texture.name;
		let description = &texture.description;
		let id = &texture.id;
		let default = &texture.default;
		let options = texture.options.ref_inner()
			.iter()
			.map(|(id, o)| (
				&**id.ref_inner(),
				option2::FrontendData::new(o, mc_version)
			))
			.collect();

		Self { name, description, id, default, options }
	}
}
