mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::TextureRuntime;
use super::option2;
use ::hashbrown::HashMap;

::nominal::nominal_mod! {
	mod nm {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub Default, inner: Option<String>);
	}
	pub mod nr {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
		nominal!(pub Default, inner: Option<option2::nr::ID>);
		nominal!(pub Options, inner: HashMap<option2::nr::ID, option2::OptionRuntime>);
	}
}
