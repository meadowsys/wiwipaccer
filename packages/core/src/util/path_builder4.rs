use crate::error::*;
use super::fs2 as fs;
use ::camino::Utf8PathBuf;
use ::std::future::{ IntoFuture, Future };
use ::std::ops::Deref;


const ROOT_MANIFEST: &str = "pack.wiwimeta";
const TEXTURE_ENTRIES_DIR: &str = "textures";
const TEXTURE_MANIFEST: &str = "texture.wiwimeta";
const OPTION_MANIFEST: &str = "option.wiwimeta";
const PROVIDER_MANIFEST: &str = "provider.wiwimeta";


impl<'h> WithRootDir<'h> {
	#[inline]
	fn _root_dir(&self) -> PathChain {
		let inner = Utf8PathBuf::from(self.root_dir);
		PathChain { inner }
	}
	#[inline]
	pub fn root_dir(&self) -> CheckPath {
		self._root_dir().check_dir()
	}

	#[inline]
	fn _root_manifest(&self) -> PathChain {
		self._root_dir().push(ROOT_MANIFEST)
	}
	#[inline]
	pub fn root_manifest(&self) -> CheckPath {
		self._root_manifest().check_file()
	}

	#[inline]
	fn _texture_entries_dir(&self) -> PathChain {
		self._root_dir().push(TEXTURE_ENTRIES_DIR)
	}
	#[inline]
	pub fn texture_entries_dir(&self) -> CheckPath {
		self._texture_entries_dir().check_dir()
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	fn _texture_dir(&self) -> PathChain {
		self._texture_entries_dir().push(self.texture_id)
	}
	#[inline]
	pub fn texture_dir(&self) -> CheckPath {
		self._texture_dir().check_dir()
	}

	#[inline]
	fn _texture_manifest(&self) -> PathChain {
		self._texture_dir().push(TEXTURE_MANIFEST)
	}
	#[inline]
	pub fn texture_manifest(&self) -> CheckPath {
		self._texture_manifest().check_file()
	}

	#[inline]
	fn _option_entries_dir(&self) -> PathChain {
		self._texture_dir()
	}
	#[inline]
	pub fn option_entries_dir(&self) -> CheckPath {
		self._option_entries_dir().check_dir()
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	fn _option_dir(&self) -> PathChain {
		self._option_entries_dir().push(self.option_id)
	}
	#[inline]
	pub fn option_dir(&self) -> CheckPath {
		self._option_dir().check_dir()
	}

	#[inline]
	fn _option_manifest(&self) -> PathChain {
		self._option_dir().push(OPTION_MANIFEST)
	}
	#[inline]
	pub fn option_manifest(&self) -> CheckPath {
		self._option_manifest().check_file()
	}

	#[inline]
	fn _provider_entries_dir(&self) -> PathChain {
		self._option_dir()
	}
	#[inline]
	pub fn provider_entries_dir(&self) -> CheckPath {
		self._provider_entries_dir().check_dir()
	}
}

impl<'h> WithProviderID<'h> {
	#[inline]
	fn _provider_dir(&self) -> PathChain {
		self._provider_entries_dir().push(self.provider_id)
	}
	#[inline]
	pub fn provider_dir(&self) -> CheckPath {
		self._provider_dir().check_dir()
	}

	#[inline]
	fn _provider_manifest(&self) -> PathChain {
		self._provider_dir().push(PROVIDER_MANIFEST)
	}
	#[inline]
	pub fn provider_manifest(&self) -> CheckPath {
		self._provider_manifest().check_file()
	}
}


pub struct Blank {
	__private: ()
}

pub struct WithRootDir<'h> {
	root_dir: &'h str
}

pub struct WithTextureID<'h> {
	root: WithRootDir<'h>,
	texture_id: &'h str
}

pub struct WithOptionID<'h> {
	texture: WithTextureID<'h>,
	option_id: &'h str
}

pub struct WithProviderID<'h> {
	option: WithOptionID<'h>,
	provider_id: &'h str
}


#[inline]
pub fn create_path_builder4() -> Blank {
	Blank { __private: () }
}

impl Blank {
	#[inline]
	pub fn with_root_dir(self, root_dir: &str) -> WithRootDir {
		WithRootDir { root_dir }
	}
}

impl<'h> WithRootDir<'h> {
	#[inline]
	pub fn root_dir_ref(&self) -> &str {
		self.root_dir
	}

	#[inline]
	pub fn with_texture_id(self, texture_id: &'h str) -> WithTextureID {
		WithTextureID { root: self, texture_id }
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub fn texture_id_ref(&self) -> &str {
		self.texture_id
	}

	#[inline]
	pub fn with_option_id(self, option_id: &'h str) -> WithOptionID {
		WithOptionID { texture: self, option_id }
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub fn option_id_ref(&self) -> &str {
		self.option_id
	}

	#[inline]
	pub fn with_provider_id(self, provider_id: &'h str) -> WithProviderID {
		WithProviderID { option: self, provider_id }
	}
}

impl<'h> WithProviderID<'h> {
	#[inline]
	pub fn provider_id_ref(&self) -> &str {
		self.provider_id
	}
}


impl<'h> Deref for WithProviderID<'h> {
	type Target = WithOptionID<'h>;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.option
	}
}

impl<'h> Deref for WithOptionID<'h> {
	type Target = WithTextureID<'h>;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.texture
	}
}

impl<'h> Deref for WithTextureID<'h> {
	type Target = WithRootDir<'h>;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.root
	}
}


struct PathChain {
	inner: Utf8PathBuf
}

impl PathChain {
	#[inline]
	fn push(mut self, path: &str) -> Self {
		self.inner.push(path);
		self
	}

	#[inline]
	fn into_string(self) -> String {
		self.inner.into_string()
	}

	#[inline]
	fn check_file(self) -> CheckPath {
		let path = self.into_string();
		let check_type = CheckType::File;
		CheckPath { path, check_type }
	}

	#[inline]
	fn check_dir(self) -> CheckPath {
		let path = self.into_string();
		let check_type = CheckType::Dir;
		CheckPath { path, check_type }
	}
}


pub struct CheckPath {
	path: String,
	check_type: CheckType
}

enum CheckType {
	File,
	Dir
}

impl CheckPath {
	#[inline]
	pub fn as_path_unchecked(&self) -> &str {
		&self.path
	}

	#[inline]
	pub fn into_path_unchecked(self) -> String {
		self.path
	}

	#[inline]
	pub fn fail_silently(self) -> CheckPathSilent {
		CheckPathSilent { inner: self }
	}
}

impl IntoFuture for CheckPath {
	type Output = Result<String, path_builder_err::CheckError>;
	type IntoFuture = impl Future<Output = Self::Output>;

	#[inline]
	fn into_future(self) -> Self::IntoFuture {
		async {
			let CheckPath { path, check_type } = self;
			let path_fn = || path.clone();

			match check_type {
				CheckType::File => {
					if fs::is_file(path_fn).await? {
						Ok(path)
					} else {
						Err(path_builder_err::not_file(path))
					}
				}
				CheckType::Dir => {
					if fs::is_dir(path_fn).await? {
						Ok(path)
					} else {
						Err(path_builder_err::not_dir(path))
					}
				}
			}
		}
	}
}

/// Silently fail (eg. return Ok(None)) if a file doesn't
/// exist or is of the wrong type
pub struct CheckPathSilent {
	inner: CheckPath
}

impl IntoFuture for CheckPathSilent {
	type Output = path_builder_err::SilentResult;
	type IntoFuture = impl Future<Output = Self::Output>;

	#[inline]
	fn into_future(self) -> Self::IntoFuture {
		use path_builder_err::CheckError::{ NotFile, NotDir };
		async {
			match self.inner.await {
				Ok(p) => {
					path_builder_err::SilentResult::Ok(p)
				}
				Err(NotFile { .. } | NotDir { .. }) => {
					path_builder_err::SilentResult::SilentFail
				}
				Err(e) => {
					path_builder_err::SilentResult::Err(e)
				}
			}
		}
	}
}
