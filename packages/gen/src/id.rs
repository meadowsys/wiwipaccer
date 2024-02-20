use ::ts_result::{ Formatter, NiceErrorMessage};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionID {
	pack_id: String,
	texture_id: String,
	option_id: String
}

impl OptionID {
	#[inline]
	pub fn builder() -> IDBuilder {
		IDBuilder::new()
	}

	#[inline]
	pub fn pack_id_ref(&self) -> &str {
		&self.pack_id
	}

	#[inline]
	pub fn texture_id_ref(&self) -> &str {
		&self.texture_id
	}

	#[inline]
	pub fn option_id_ref(&self) -> &str {
		&self.option_id
	}
}

pub struct IDBuilder {
	__private: ()
}

impl IDBuilder {
	#[inline]
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self { __private: () }
	}

	#[inline]
	pub fn pack_id(self, pack_id: &str) -> WithPackID {
		WithPackID { pack_id }
	}
}

pub struct WithPackID<'h> {
	pack_id: &'h str
}

impl<'h> WithPackID<'h> {
	#[inline]
	pub fn texture_id(self, texture_id: &'h str) -> WithTextureID {
		let Self { pack_id } = self;
		WithTextureID { pack_id, texture_id }
	}
}

pub struct WithTextureID<'h> {
	pack_id: &'h str,
	texture_id: &'h str
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub fn option_id(self, option_id: &'h str) -> WithOptionID {
		let Self { pack_id, texture_id } = self;
		WithOptionID { pack_id, texture_id, option_id }
	}
}

pub struct WithOptionID<'h> {
	pack_id: &'h str,
	texture_id: &'h str,
	option_id: &'h str
}

impl<'h> WithOptionID<'h> {
	pub fn build(self) -> Result<OptionID, Error> {
		let Self { pack_id, texture_id, option_id } = self;

		if pack_id.contains(':') {
			return Err(Error::PackIDInvalid(pack_id.into()))
		}

		if texture_id.contains(':') {
			return Err(Error::TextureIDInvalid(texture_id.into()))
		}

		if option_id.contains(':') {
			return Err(Error::OptionIDInvalid(option_id.into()))
		}

		let pack_id = pack_id.into();
		let texture_id = texture_id.into();
		let option_id = option_id.into();

		Ok(OptionID { pack_id, texture_id, option_id })
	}
}

#[derive(Debug)]
pub enum Error {
	PackIDInvalid(String),
	TextureIDInvalid(String),
	OptionIDInvalid(String)
}

impl NiceErrorMessage for Error {
	fn fmt(&self, f: &mut Formatter) {
		use Error::*;
		let (name, id) = match self {
			PackIDInvalid(pack_id) => { ("Pack ID", pack_id) }
			TextureIDInvalid(texture_id) => { ("Texture ID", texture_id) }
			OptionIDInvalid(option_id) => { ("Option ID", option_id) }
		};

		f.write_line_args(format_args!("{name} is invalid; it cannot contain a \":\" character:"));
		f.with_indent(|f| f.write_str(id));
	}
}
