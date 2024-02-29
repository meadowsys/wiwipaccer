use ::boxed::Boxed as _;
use ::std::fmt::{ self, Display };
use ::ts_result::{ Formatter, NiceErrorMessage, impl_display };

/// An ID for an option, consisting of the pack its from, the texture its for,
/// and its own option-specific ID
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

impl Display for OptionID {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { pack_id, texture_id, option_id } = self;
		write!(f, "{pack_id}:{texture_id}:{option_id}")
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

#[derive(Debug)]
pub struct Error {
	pack_id: Option<Box<ComponentError>>,
	texture_id: Option<Box<ComponentError>>,
	option_id: Option<Box<ComponentError>>,
}

#[derive(Debug)]
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
			f.next_line();
		}

		if let Some(texture_id) = &self.texture_id {
			f.write_line("Texture ID is invalid");
			f.with_indent(|f| f.fmt(&**texture_id));
			f.next_line();
		}

		if let Some(option_id) = &self.option_id {
			f.write_line("Option ID is invalid");
			f.with_indent(|f| f.fmt(&**option_id));
			f.next_line();
		}

		f.write_str("ID components are only allowed to contain loweralpha, numeric, and dash characters");
	}
}

impl NiceErrorMessage for ComponentError {
	fn fmt(&self, f: &mut Formatter) {
		let Self { component, invalid_chars } = self;

		f.write_line_args(format_args!("provided component: {component}"));
		f.write_line("invalid characters found:");
		f.with_indent(|f| {
			invalid_chars.iter().copied().for_each(|c| {
				f.write_str("- '");
				f.write_char(c);
				f.write_char('\'');
				f.next_line();
			});
			f.undo_next_line();
		});
	}
}

impl_display!(Error);
impl ::ts_result::Error for Error {}

/// ID components are only allowed to contain loweralpha, numeric, and dash characters
#[inline]
fn invalid_chars(id_component: &str) -> Option<Vec<char>> {
	#[inline]
	fn char_is_valid(c: char) -> bool {
		matches!(c, 'a'..='z' | '0'..='9' | '-')
	}

	let vec = id_component.chars()
		.filter(|c| !char_is_valid(*c))
		.collect::<Vec<_>>();
	if vec.is_empty() { None } else { Some(vec) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builder() {
		// good
		OptionID::builder()
			.pack_id("lt")
			.texture_id("stone")
			.option_id("random")
			.build()
			.unwrap();

		// good
		// also lt-vh doesn't actually exist at the time of test writing,
		// but i do wish it did :p soon™ perhaps? ~vt
		OptionID::builder()
			.pack_id("lt-vh")
			.texture_id("crystal")
			.option_id("pony")
			.build()
			.unwrap();

		// option ID contains exclamation mark
		OptionID::builder()
			.pack_id("lt")
			.texture_id("stone")
			.option_id("invalidid!")
			.build()
			.unwrap_err();

		// pack ID contains spaces
		OptionID::builder()
			.pack_id("another invalid id")
			.texture_id("stone")
			.option_id("random")
			.build()
			.unwrap_err();

		// texture ID contains whatever the accents in ùwú are called hee
		OptionID::builder()
			.pack_id("lt")
			.texture_id("aha ùwú")
			.option_id("random")
			.build()
			.unwrap_err();
	}

	#[test]
	fn ref_fns() {
		let lt_stone = OptionID::builder()
			.pack_id("lt")
			.texture_id("stone")
			.option_id("random")
			.build()
			.unwrap();

		assert_eq!("lt", lt_stone.pack_id_ref());
		assert_eq!("stone", lt_stone.texture_id_ref());
		assert_eq!("random", lt_stone.option_id_ref());
	}

	#[test]
	fn display() {
		let lt_stone = OptionID::builder()
			.pack_id("lt")
			.texture_id("stone")
			.option_id("random")
			.build()
			.unwrap();

		assert_eq!("lt:stone:random", lt_stone.to_string());
	}

	#[test]
	fn error_message() {
		let expected = include_str!("../test/fixtures/error-message.txt");
		let error = OptionID::builder()
			.pack_id("l&t")
			.texture_id("aha ùwú")
			.option_id("ra andom")
			.build()
			.unwrap_err()
			.to_error_message();
		assert_eq!(expected, &*error);
	}
}
