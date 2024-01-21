mod root;
mod with_texture;
mod with_option;
mod with_version;

use crate::nom as n;
pub use root::Root;
pub use with_texture::WithTexture;
pub use with_option::WithOption;
pub use with_version::WithVersion;

#[inline]
pub fn path_builder(root_dir: &n::global::RootDirPath) -> Root {
	path_builder2(root_dir.ref_inner())
}

#[inline]
pub fn path_builder2(root_dir: &str) -> Root {
	Root { root_dir }
}
