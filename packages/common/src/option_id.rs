use ::boxed::Boxed as _;
use ::ts_result::{ Formatter, NiceErrorMessage };

/// An ID for an option, consisting of the pack its from, the texture its for,
/// and its own option-specific ID
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

		let mut error = Error::blank();

		if let Some(invalid_chars) = invalid_chars(pack_id) {
			error.pack_id = Some(ComponentError {
				component: pack_id.into(),
				invalid_chars
			}.boxed());
			// return Err(Error::PackIDInvalid(pack_id.into()))
		}

		if let Some(invalid_chars) = invalid_chars(texture_id) {
			error.texture_id = Some(ComponentError {
				component: texture_id.into(),
				invalid_chars
			}.boxed());
		}

		if let Some(invalid_chars) = invalid_chars(option_id) {
			error.option_id = Some(ComponentError {
				component: option_id.into(),
				invalid_chars
			}.boxed());
		}

		if error.contains_error() { return Err(error) }

		let pack_id = pack_id.into();
		let texture_id = texture_id.into();
		let option_id = option_id.into();

		Ok(OptionID { pack_id, texture_id, option_id })
	}
}

pub struct Error {
	pack_id: Option<Box<ComponentError>>,
	texture_id: Option<Box<ComponentError>>,
	option_id: Option<Box<ComponentError>>,
}

struct ComponentError {
	component: String,
	invalid_chars: Vec<char>
}

impl Error {
	fn blank() -> Self {
		let pack_id = None;
		let texture_id = None;
		let option_id = None;
		Self { pack_id, texture_id, option_id }
	}

	fn contains_error(&self) -> bool {
		if self.pack_id.is_some() { return true }
		if self.texture_id.is_some() { return true }
		if self.option_id.is_some() { return true }
		false
	}
}

impl NiceErrorMessage for Error {
	fn fmt(&self, f: &mut Formatter) {
		if let Some(pack_id) = &self.pack_id {
			f.write_line("Pack ID is invalid");
			f.with_indent(|f| f.fmt(&**pack_id));
		}

		if let Some(texture_id) = &self.texture_id {
			f.write_line("Texture ID is invalid");
			f.with_indent(|f| f.fmt(&**texture_id));
		}

		if let Some(option_id) = &self.option_id {
			f.write_line("Option ID is invalid");
			f.with_indent(|f| f.fmt(&**option_id));
		}
	}
}

impl NiceErrorMessage for ComponentError {
	fn fmt(&self, f: &mut Formatter) {
		let Self { component, invalid_chars } = self;
		let [c1, rest @ ..] = &**invalid_chars else {
			// enforced by invalid_chars, it will not provide
			// ComponentErrors are created by WithOptionID::build, which uses
			// invalid_chars to check for invalid characters, and the result vec from
			// there is passed in unmodified. invalid_chars checks for the vec to be
			// length 1 or more, and that is the vec we are destructuring here,
			// so it will always succeed
			unreachable!()
		};

		f.write_args(format_args!("provided component: {component}"));
		f.write_char(*c1);
		rest.iter().copied().for_each(|c| {
			f.write_str(", ");
			f.write_char(c);
		});
	}
}

/// ID components are only allowed to contain alphanumeric and dash chars
#[inline]
fn invalid_chars(id_component: &str) -> Option<Vec<char>> {
	#[inline]
	fn char_is_valid(c: char) -> bool {
		matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-')
	}

	let vec = id_component.chars()
		.filter(|c| char_is_valid(*c))
		.collect::<Vec<_>>();
	if vec.is_empty() { None } else { Some(vec) }
}
