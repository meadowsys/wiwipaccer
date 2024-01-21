mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::OptionRuntime;
use super::version2;
use ::hashbrown::HashMap;

::nominal::nominal_mod! {
	mod nm {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
	}
	pub mod nr {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
		nominal!(pub Versions, inner: HashMap<version2::nr::ID, version2::VersionRuntime>);
	}
}
