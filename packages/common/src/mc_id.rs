use crate::error::{ ComponentError, MinecraftIDError };
use ::boxed::Boxed as _;
use ::std::fmt::{ self, Display };

/// Minecraft block/item ID
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MinecraftID {
	ns: String,
	id: String
}

impl MinecraftID {
	#[inline]
	pub fn builder() -> IDBuilder {
		IDBuilder::new()
	}

	#[inline]
	pub fn ns_ref(&self) -> &str {
		&self.ns
	}

	#[inline]
	pub fn id_ref(&self) -> &str {
		&self.id
	}
}

impl Display for MinecraftID {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { ns, id } = self;
		write!(f, "{ns}:{id}")
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
	pub fn namespace(self, ns: &str) -> WithNamespace {
		WithNamespace { ns }
	}
}

pub struct WithNamespace<'h> {
	ns: &'h str
}

impl<'h> WithNamespace<'h> {
	#[inline]
	pub fn id(self, id: &'h str) -> WithID {
		let Self { ns } = self;
		WithID { ns, id }
	}
}

pub struct WithID<'h> {
	ns: &'h str,
	id: &'h str
}

impl<'h> WithID<'h> {
	pub fn build(self) -> Result<MinecraftID, MinecraftIDError> {
		let Self { ns, id } = self;

		let mut error = MinecraftIDError::blank();

		if let Some(invalid_chars) = invalid_chars(ns) {
			error.ns = Some(ComponentError {
				component: ns.into(),
				invalid_chars
			}.boxed());
		}

		if let Some(invalid_chars) = invalid_chars(id) {
			error.id = Some(ComponentError {
				component: id.into(),
				invalid_chars
			}.boxed());
		}

		if error.contains_error() { return Err(error) }

		let ns = ns.into();
		let id = id.into();

		Ok(MinecraftID { ns, id })
	}
}

/// Minecraft ID components are only allowed to contain loweralpha, numeric, and
/// underscore characters (I think? ~vt)
pub(crate) fn invalid_chars(id_component: &str) -> Option<Vec<char>> {
	#[inline]
	fn char_is_valid(c: char) -> bool {
		matches!(c, 'a'..='z' | '0'..='9' | '_')
	}

	let iter = id_component.chars()
		.filter(|c| !char_is_valid(*c));

	// dedupe, but preserving order
	let (hint_lower, hint_upper) = iter.size_hint();
	let vec = Vec::with_capacity(hint_upper.unwrap_or(hint_lower));
	let vec = iter.fold(vec, |mut acc, curr| {
		if !acc.contains(&curr) { acc.push(curr) }
		acc
	});

	if vec.is_empty() { None } else { Some(vec) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builder() {
		// good
		MinecraftID::builder()
			.namespace("minecraft")
			.id("stone")
			.build()
			.unwrap();

		// good
		MinecraftID::builder()
			.namespace("the_vault")
			.id("vault_diamond")
			.build()
			.unwrap();

		// ID contains exclamation mark
		MinecraftID::builder()
			.namespace("minecraft")
			.id("aha invalid!!!!!!!!")
			.build()
			.unwrap_err();


		// namespace contains spaces
		MinecraftID::builder()
			.namespace("name space")
			.id("stone")
			.build()
			.unwrap_err();

		// ID contains whatever the accents in ùwú are called hee
		MinecraftID::builder()
			.namespace("lt")
			.id("aha ùwú")
			.build()
			.unwrap_err();
	}

	#[test]
	fn ref_fns() {
		let vault_diamond = MinecraftID::builder()
			.namespace("the_vault")
			.id("vault_diamond")
			.build()
			.unwrap();

		assert_eq!("the_vault", vault_diamond.ns_ref());
		assert_eq!("vault_diamond", vault_diamond.id_ref());
	}

	#[test]
	fn display() {
		let vault_diamond = MinecraftID::builder()
			.namespace("the_vault")
			.id("vault_diamond")
			.build()
			.unwrap()
			.to_string();
		assert_eq!("the_vault:vault_diamond", &*vault_diamond);
	}
}
