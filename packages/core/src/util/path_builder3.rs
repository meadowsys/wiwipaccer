use super::error::*;
use super::fs;
use ::camino::Utf8PathBuf;
use ::std::ops::Deref;

// -- consts --

const ROOT_META_FILENAME: &str = "pack.wiwimeta";

const TEXTURE_ENTRIES_DIR: &str = "textures";
const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

const OPTION_META_FILENAME: &str = "option.wiwimeta";

const VERSION_META_FILENAME: &str = "version.wiwimeta";

// -- structs --

// lol, not really necessary at all, but eh lel
pub struct Blank {
	__private: ()
}

#[derive(Clone, Debug)]
pub struct WithRootDir<'h> {
	root_dir: &'h str
}

#[derive(Clone, Debug)]
pub struct WithTextureID<'h> {
	with_root_dir: WithRootDir<'h>,
	texture_id: &'h str
}

#[derive(Clone, Debug)]
pub struct WithOptionID<'h> {
	with_texture_id: WithTextureID<'h>,
	option_id: &'h str
}

#[derive(Clone, Debug)]
pub struct WithVersionID<'h> {
	with_option_id: WithOptionID<'h>,
	version_id: &'h str
}

// -- creation / upgrade fns --

pub fn create_path_builder3() -> Blank {
	Blank { __private: () }
}

impl Blank {
	pub fn with_root_dir(self, root_dir: &str) -> WithRootDir {
		WithRootDir { root_dir }
	}
}

impl<'h> WithRootDir<'h> {
	pub fn with_texture_id(self, texture_id: &'h str) -> WithTextureID {
		WithTextureID { with_root_dir: self, texture_id }
	}
}

impl<'h> WithTextureID<'h> {
	pub fn with_option_id(self, option_id: &'h str) -> WithOptionID {
		WithOptionID { with_texture_id: self, option_id }
	}
}

impl<'h> WithOptionID<'h> {
	pub fn with_version_id(self, version_id: &'h str) -> WithVersionID {
		WithVersionID { with_option_id: self, version_id }
	}
}

// -- deref impls --

impl<'h> Deref for WithTextureID<'h> {
	type Target = WithRootDir<'h>;
	fn deref(&self) -> &Self::Target {
		&self.with_root_dir
	}
}

impl<'h> Deref for WithOptionID<'h> {
	type Target = WithTextureID<'h>;
	fn deref(&self) -> &Self::Target {
		&self.with_texture_id
	}
}

impl<'h> Deref for WithVersionID<'h> {
	type Target = WithOptionID<'h>;
	fn deref(&self) -> &Self::Target {
		&self.with_option_id
	}
}

// -- logic impls (private fns) --

impl<'h> WithRootDir<'h> {
	fn _root_dir(&self) -> Utf8PathBuf {
		Utf8PathBuf::from(self.root_dir)
	}

	fn _root_meta_filename(&self) -> Utf8PathBuf {
		let mut path = self._root_dir();
		path.push(ROOT_META_FILENAME);
		path
	}

	fn _texture_entries_dir(&self) -> Utf8PathBuf {
		let mut path = self._root_dir();
		path.push(TEXTURE_ENTRIES_DIR);
		path
	}
}

impl<'h> WithTextureID<'h> {
	fn _texture_dir(&self) -> Utf8PathBuf {
		let mut path = self._texture_entries_dir();
		path.push(self.texture_id);
		path
	}

	fn _texture_meta_filename(&self) -> Utf8PathBuf {
		let mut path = self._texture_dir();
		path.push(TEXTURE_META_FILENAME);
		path
	}

	fn _option_entries_dir(&self) -> Utf8PathBuf {
		self._texture_dir()
	}
}

impl<'h> WithOptionID<'h> {
	fn _option_dir(&self) -> Utf8PathBuf {
		let mut path = self._texture_dir();
		path.push(self.option_id);
		path
	}

	fn _option_meta_filename(&self) -> Utf8PathBuf {
		let mut path = self._option_dir();
		path.push(OPTION_META_FILENAME);
		path
	}

	fn _version_entries_dir(&self) -> Utf8PathBuf {
		self._option_dir()
	}
}

impl<'h> WithVersionID<'h> {
	fn _version_dir(&self) -> Utf8PathBuf {
		let mut path = self._option_dir();
		path.push(self.version_id);
		path
	}

	fn _version_meta_filename(&self) -> Utf8PathBuf {
		let mut path = self._version_dir();
		path.push(VERSION_META_FILENAME);
		path
	}
}

// -- public interface --
