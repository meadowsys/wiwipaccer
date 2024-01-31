mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::{
	PackRuntime,
	Dependency,
	DependencyResolver,
	DependencyResult,
	FrontendData
};
use super::texture2;
use ::hashbrown::HashMap;

::nominal::nominal_mod! {
	mod nm {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
		nominal!(pub Version, inner: Option<String>);
		nominal!(pub VersionReq, inner: String);
		nominal!(pub Dependencies, inner: Option<HashMap<ID, VersionReq>>);
	}
	pub mod nr {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
		nominal!(pub Dir, inner: String);
		nominal!(pub Version, inner: Option<::semver::Version>);
		nominal!(pub Dependencies, inner: HashMap<ID, ::semver::VersionReq>);
		nominal!(pub Textures, inner: HashMap<texture2::nr::ID, texture2::TextureRuntime>);
	}
}
