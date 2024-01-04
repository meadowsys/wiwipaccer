//! provides ron serialise/deserialise functions using wiwipaccer's default ron config

use crate::error::*;
use ::ron::extensions::Extensions;
pub use ::ron::ser::PrettyConfig;
use ::serde::{ Deserialize, Serialize };

#[inline]
fn extensions() -> Extensions {
	Extensions::IMPLICIT_SOME
	| Extensions::UNWRAP_NEWTYPES
	| Extensions::UNWRAP_VARIANT_NEWTYPES
}

#[inline]
fn pretty_config() -> PrettyConfig {
	PrettyConfig::new()
		.indentor("\t".into())
		.new_line("\n".into())
}

/// Deserialize struct from a ron string, enabling some extensions
pub fn from_str<'h, T>(s: &'h str) -> Result<T>
where
	T: Deserialize<'h>
{
	ron::Options::default()
		.with_default_extension(extensions())
		.from_str(s)
		.map_err(Into::into)
		.map_err(Error)
}

/// Serialize struct to a ron string, enabling some extensions, and not using
/// pretty formatting (minified)
pub fn to_string_minified<T>(value: &T) -> Result<String>
where
	T: ?Sized + Serialize
{
	ron::Options::default()
		.with_default_extension(extensions())
		.to_string(value)
		.map_err(Into::into)
		.map_err(Error)
}

/// Serialize struct to a ron string, enabling some extensions, and using default
/// wiwipaccer formatting options
#[inline]
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
	T: ?Sized + Serialize
{
	to_string_pretty_custom(value, pretty_config())
}

/// Serialize struct to a ron string, enabling some extensions, but passing
/// default wiwipaccer formatting options through a closure to allow for modification
#[inline]
pub fn to_string_pretty_extend<T, F>(value: &T, f: F) -> Result<String>
where
	T: ?Sized + Serialize,
	F: FnOnce(PrettyConfig) -> PrettyConfig
{
	to_string_pretty_custom(value, f(pretty_config()))
}

/// Serialize struct to a ron string, enabling some extensions, and allowing to pass in
/// fully custom formatting options
pub fn to_string_pretty_custom<T>(value: &T, config: PrettyConfig) -> Result<String>
where
	T: ?Sized + Serialize
{
	ron::Options::default()
		.with_default_extension(extensions())
		.to_string_pretty(value, config)
		.map_err(Into::into)
		.map_err(Error)
}
