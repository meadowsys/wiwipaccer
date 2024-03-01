use crate::error::{ ComponentError, OptionIDError, invalid_chars };
use ::boxed::Boxed as _;
use ::std::fmt::{ self, Display };

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
	pub fn build(self) -> Result<OptionID, OptionIDError> {
		let Self { pack_id, texture_id, option_id } = self;

		let mut error = OptionIDError::blank();

		if let Some(invalid_chars) = invalid_chars(pack_id) {
			error.pack_id = Some(ComponentError {
				component: pack_id.into(),
				invalid_chars
			}.boxed());
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
			.unwrap()
			.to_string();
		assert_eq!("lt:stone:random", &*lt_stone);
	}
}
