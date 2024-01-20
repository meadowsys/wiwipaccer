mod error;
mod meta;
mod rt;

pub use self::error::Error;
use ::hashbrown::HashMap;

::nominal::nominal_mod! {
	mod nm {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
		nominal!(pub Version, inner: String);
		nominal!(pub VersionReq, inner: String);
		nominal!(pub Dependencies, inner: Option<HashMap<ID, VersionReq>>);
	}
}
