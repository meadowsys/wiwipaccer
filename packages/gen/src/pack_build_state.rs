use ::hashbrown::HashMap;
use ::wiwipaccer_common::OptionID;

pub struct PackBuildState {
	packs: HashMap<String, PackState>
}

impl PackBuildState {
	pub fn new() -> Self {
		let packs = HashMap::new();
		Self { packs }
	}

	pub fn use_pack(&mut self, pack_id: &str) -> &mut PackState {
		self.packs.entry_ref(pack_id)
			.or_insert_with(PackState::new)
	}

	pub fn use_option(&mut self, option_id: OptionID) -> &mut OptionState {
		self
			.use_pack(option_id.pack_id_ref())
			.use_texture(option_id.texture_id_ref())
			.use_option(option_id.option_id_ref())
	}
}

pub struct PackState {
	textures: HashMap<String, TextureState>
}

impl PackState {
	pub fn new() -> Self {
		let textures = HashMap::new();
		Self { textures }
	}

	pub fn use_texture(&mut self, texture_id: &str) -> &mut TextureState {
		self.textures.entry_ref(texture_id)
			.or_insert_with(TextureState::new)
	}
}

pub struct TextureState {
	options: HashMap<String, OptionState>
}

impl TextureState {
	pub fn new() -> Self {
		let options = HashMap::new();
		Self { options }
	}

	pub fn use_option(&mut self, option_id: &str) -> &mut OptionState {
		self.options.entry_ref(option_id)
			.or_insert_with(OptionState::new)
	}
}

pub struct OptionState {}

impl OptionState {
	pub fn new() -> Self {
		Self {}
	}
}
