use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::WithTexture;
use super::WithVersion;

const OPTION_META_FILENAME: &str = "option.wiwimeta";

pub struct WithOption<'r, 't, 'o> {
	pub(super) with_texture: WithTexture<'r, 't>,
	pub(super) option_id: &'o n::option::ID
}

impl<'r, 't, 'o> WithOption<'r, 't, 'o> {
	#[inline]
	pub(super) fn _option_dir(&self) -> Utf8PathBuf {
		let mut path = self.with_texture._texture_dir();
		path.push(self.option_id.ref_inner());
		path
	}

	#[inline]
	pub fn root_dir(&self) -> n::global::RootDirPath {
		self.with_texture.root_dir()
	}

	#[inline]
	pub fn root_manifest(&self) -> n::global::RootManifestPath {
		self.with_texture.root_manifest()
	}

	#[inline]
	pub fn texture_dir(&self) -> n::global::TextureDirPath {
		self.with_texture.texture_dir()
	}

	#[inline]
	pub fn texture_manifest(&self) -> n::global::TextureManifestPath {
		self.with_texture.texture_manifest()
	}

	#[inline]
	pub fn option_dir(&self) -> n::global::OptionDirPath {
		let path = self._option_dir();
		n::global::OptionDirPath::new(path.into_string())
	}

	#[inline]
	pub fn option_manifest(&self) -> n::global::OptionManifestPath {
		let mut path = self._option_dir();
		path.push(OPTION_META_FILENAME);
		n::global::OptionManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_version(self, version_id: &n::version::ID) -> WithVersion {
		let with_option = self;
		WithVersion { with_option, version_id }
	}
}
