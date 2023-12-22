use crate::error::*;
use ron::extensions::Extensions;
use serde::{ Deserialize, Serialize };

#[inline]
fn enabled_extensions() -> Extensions {
	Extensions::IMPLICIT_SOME
	| Extensions::UNWRAP_NEWTYPES
	| Extensions::UNWRAP_VARIANT_NEWTYPES
}

pub fn from_str<'h, T>(s: &'h str) -> Result<T>
where
	T: Deserialize<'h>
{
	ron::Options::default()
		.with_default_extension(enabled_extensions())
		.from_str(s)
		.map_err(Into::into)
}
