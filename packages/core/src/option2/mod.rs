mod error;
mod meta;
mod rt;

pub use self::error::Error;

::nominal::nominal_mod! {
	mod nm {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
	}
}
