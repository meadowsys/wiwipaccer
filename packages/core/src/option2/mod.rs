mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::OptionRuntime;
use super::option_provider2;
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
		nominal!(pub OptionProviders, inner: HashMap<option_provider2::nr::ID, option_provider2::OptionProviderRuntime>);
	}
}
