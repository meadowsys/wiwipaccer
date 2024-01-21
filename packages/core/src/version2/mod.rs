mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::{ PackVersionSpecRuntime, VersionRuntime };

::nominal::nominal_mod! {
	pub mod nr {
		nominal!(pub ID, inner: String);
	}
}
