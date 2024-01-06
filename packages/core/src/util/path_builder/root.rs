use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::WithTexture;

const PACK_META_FILENAME: &str = "pack.wiwimeta";

pub struct Root<'r> {
	pub(super) root_dir: &'r n::global::RootDirPath
}

impl<'r> Root<'r> {
	#[inline]
	pub(super) fn _root_dir(&self) -> Utf8PathBuf {
		Utf8PathBuf::from(self.root_dir.clone().into_inner())
	}

	#[inline]
	pub fn root_dir(&self) -> n::global::RootDirPath {
		self.root_dir.clone()
	}

	#[inline]
	pub fn root_manifest(&self) -> n::global::RootManifestPath {
		let mut path = self._root_dir();
		path.push(PACK_META_FILENAME);
		n::global::RootManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_texture(self, texture_id: &n::texture::ID) -> WithTexture {
		let root = self;
		WithTexture { root, texture_id }
	}
}
