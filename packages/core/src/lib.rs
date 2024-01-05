pub mod option;
pub mod pack;
pub mod texture;
pub mod util;
pub mod workspace;

::nominal::nominal_mod! {
	pub mod nom {
		nominal!(pub RootDir, inner: String);
		nominal!(pub PackID, inner: String);
	}
}
