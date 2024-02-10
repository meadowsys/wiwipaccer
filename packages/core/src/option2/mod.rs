mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::{ OptionRuntime, FrontendData };
use super::provider2;
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
		nominal!(pub Providers, inner: HashMap<provider2::nr::ID, provider2::ProviderRuntime>);
	}
}
