use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::Root;
use super::WithOption;

const TEXTURES_DIR: &str = "textures";
const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

pub struct WithTexture<'r, 't> {
	pub(super) root: Root<'r>,
	pub(super) texture_id: &'t n::texture::ID
}

impl<'r, 't> WithTexture<'r, 't> {
	#[inline]
	pub(super) fn _texture_dir(&self) -> Utf8PathBuf {
		let mut path = self.root._root_dir();
		path.push(TEXTURES_DIR);
		path.push(self.texture_id.ref_inner());

		path
	}

	#[inline]
	pub fn root_dir(&self) -> n::global::RootDirPath {
		self.root.root_dir()
	}

	#[inline]
	pub fn root_manifest(&self) -> n::global::RootManifestPath {
		self.root.root_manifest()
	}

	#[inline]
	pub fn texture_dir(&self) -> n::global::TextureDirPath {
		let path = self._texture_dir();
		n::global::TextureDirPath::new(path.into_string())
	}

	#[inline]
	pub fn texture_manifest(&self) -> n::global::TextureManifestPath {
		let mut path = self._texture_dir();
		path.push(TEXTURE_META_FILENAME);
		n::global::TextureManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_option(self, option_id: &n::option::ID) -> WithOption {
		let with_texture = self;
		WithOption { with_texture, option_id }
	}
}
