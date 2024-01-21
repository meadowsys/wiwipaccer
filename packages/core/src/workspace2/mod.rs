mod error;
mod meta;
mod rt;

pub use self::error::Error;
pub use self::rt::WorkspaceRuntime;
use super::pack2;
use ::hashbrown::HashMap;

::nominal::nominal_mod! {
	mod nm {
		nominal!(pub Name, inner: String);
		nominal!(pub Packs, inner: Vec<String>);
	}
	pub mod nr {
		nominal!(pub Name, inner: String);
		nominal!(pub Packs, inner: HashMap<pack2::nr::ID, pack2::PackRuntime>);
		nominal!(pub PackIDs, inner: Vec<pack2::nr::ID>);
	}
}
