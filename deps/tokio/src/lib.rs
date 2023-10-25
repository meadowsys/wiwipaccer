#[cfg(feature = "dynamic")]
pub use ::tokio_dynamic::*;

#[cfg(feature = "static")]
pub use ::tokio_static::*;

#[cfg(all(feature = "dynamic", feature = "static"))]
compile_error!("must have exactly one of `dynamic` or `static` features enabled; found both enabled");

#[cfg(not(any(
	feature = "dynamic",
	feature = "static"
)))]
compile_error!("must have exactly one of `dynamic` or `static` features enabled; found none enabled");
