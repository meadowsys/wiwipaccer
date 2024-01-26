use super::error::*;
use super::fs;
use ::camino::Utf8PathBuf;
use ::std::future::Future;
use ::std::ops::ControlFlow::{ self, Break, Continue };
use ::std::ops::{ Deref, FromResidual, Try };

// -- consts --

const ROOT_MANIFEST: &str = "pack.wiwimeta";

const TEXTURE_ENTRIES_DIR: &str = "textures";
const TEXTURE_MANIFEST: &str = "texture.wiwimeta";

const OPTION_MANIFEST: &str = "option.wiwimeta";

const VERSION_MANIFEST: &str = "version.wiwimeta";

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

#[inline]
pub fn create_path_builder3() -> Blank {
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
	pub fn with_texture_id(self, texture_id: &'h str) -> WithTextureID {
		WithTextureID { with_root_dir: self, texture_id }
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub fn with_option_id(self, option_id: &'h str) -> WithOptionID {
		WithOptionID { with_texture_id: self, option_id }
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub fn with_version_id(self, version_id: &'h str) -> WithVersionID {
		WithVersionID { with_option_id: self, version_id }
	}
}

// // -- meta fns --

impl<'h> WithRootDir<'h> {
	#[inline]
	pub fn root_dir(&self) -> &str {
		self.root_dir
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub fn texture_id(&self) -> &str {
		self.texture_id
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub fn option_id(&self) -> &str {
		self.option_id
	}
}

impl<'h> WithVersionID<'h> {
	#[inline]
	pub fn version_id(&self) -> &str {
		self.version_id
	}
}

// -- deref impls --

impl<'h> Deref for WithTextureID<'h> {
	type Target = WithRootDir<'h>;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.with_root_dir
	}
}

impl<'h> Deref for WithOptionID<'h> {
	type Target = WithTextureID<'h>;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.with_texture_id
	}
}

impl<'h> Deref for WithVersionID<'h> {
	type Target = WithOptionID<'h>;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.with_option_id
	}
}

// -- logic impls (private fns) --

impl<'h> WithRootDir<'h> {
	#[inline]
	fn _root_dir(&self) -> Utf8PathBuf {
		Utf8PathBuf::from(self.root_dir)
	}

	#[inline]
	fn _root_manifest(&self) -> Utf8PathBuf {
		let mut path = self._root_dir();
		path.push(ROOT_MANIFEST);
		path
	}

	#[inline]
	fn _texture_entries_dir(&self) -> Utf8PathBuf {
		let mut path = self._root_dir();
		path.push(TEXTURE_ENTRIES_DIR);
		path
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	fn _texture_dir(&self) -> Utf8PathBuf {
		let mut path = self._texture_entries_dir();
		path.push(self.texture_id);
		path
	}

	#[inline]
	fn _texture_manifest(&self) -> Utf8PathBuf {
		let mut path = self._texture_dir();
		path.push(TEXTURE_MANIFEST);
		path
	}

	#[inline]
	fn _option_entries_dir(&self) -> Utf8PathBuf {
		self._texture_dir()
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	fn _option_dir(&self) -> Utf8PathBuf {
		let mut path = self._texture_dir();
		path.push(self.option_id);
		path
	}

	#[inline]
	fn _option_manifest(&self) -> Utf8PathBuf {
		let mut path = self._option_dir();
		path.push(OPTION_MANIFEST);
		path
	}

	#[inline]
	fn _version_entries_dir(&self) -> Utf8PathBuf {
		self._option_dir()
	}
}

impl<'h> WithVersionID<'h> {
	#[inline]
	fn _version_dir(&self) -> Utf8PathBuf {
		let mut path = self._option_dir();
		path.push(self.version_id);
		path
	}

	#[inline]
	fn _version_manifest(&self) -> Utf8PathBuf {
		let mut path = self._version_dir();
		path.push(VERSION_MANIFEST);
		path
	}
}

#[inline]
async fn check_path<F, Fu, Fe>(
	path_name: &str,
	f: F,
	f_err: Fe,
	path: Utf8PathBuf
) -> Result<String>
where
	F: FnOnce(String) -> Fu,
	Fu: Future<Output = Result<bool>>,
	Fe: FnOnce(String, String) -> Error
{
	let path = path.into_string();
	if f(path.clone()).await? {
		Ok(path)
	} else {
		Err(f_err(path, path_name.into()))
	}
}

#[inline]
async fn check_dir(path_name: &str, path: Utf8PathBuf) -> Result<String> {
	#[inline]
	fn f_err(path: String, path_name: String) -> Error {
		Error::PathIsNotDir { path, path_name }
	}

	check_path(path_name, fs::is_dir2, f_err, path).await
}

#[inline]
async fn check_file(path_name: &str, path: Utf8PathBuf) -> Result<String> {
	#[inline]
	fn f_err(path: String, path_name: String) -> Error {
		Error::PathIsNotFile { path, path_name }
	}

	check_path(path_name, fs::is_file2, f_err, path).await
}

#[inline]
async fn check_silent_fail<'h, F, Fu>(
	f: F,
	path_name: &'h str,
	path: Utf8PathBuf
) -> SilentFailingPath
where
	F: FnOnce(&'h str, Utf8PathBuf) -> Fu,
	Fu: Future<Output = Result<String>>
{
	match f(path_name, path).await {
		Ok(p) => { SilentFailingPath::Ok(p) }
		Err(e) if e.should_silent_fail() => { SilentFailingPath::SilentFail }
		Err(e) => { SilentFailingPath::Err(e) }
	}
}

#[inline]
async fn check_dir_silent_fail(path_name: &str, path: Utf8PathBuf) -> SilentFailingPath {
	check_silent_fail(check_dir, path_name, path).await
}

#[inline]
async fn check_file_silent_fail(path_name: &str, path: Utf8PathBuf) -> SilentFailingPath {
	check_silent_fail(check_file, path_name, path).await
}

// -- public interface --

impl<'h> WithRootDir<'h> {
	#[inline]
	pub async fn checked_root_dir(&self) -> Result<String> {
		check_dir(
			"root dir",
			self._root_dir()
		).await
	}

	#[inline]
	pub async fn checked_root_manifest(&self) -> Result<String> {
		check_file(
			"root manifest",
			self._root_manifest()
		).await
	}

	#[inline]
	pub async fn checked_texture_entries_dir(&self) -> Result<String> {
		check_dir(
			"texture entries dir",
			self._texture_entries_dir()
		).await
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub async fn checked_texture_dir(&self) -> Result<String> {
		check_dir(
			"texture dir",
			self._texture_dir()
		).await
	}

	#[inline]
	pub async fn checked_texture_manifest(&self) -> Result<String> {
		check_file(
			"texture manifest",
			self._texture_manifest()
		).await
	}

	#[inline]
	pub async fn checked_option_entries_dir(&self) -> Result<String> {
		check_dir(
			"option entries dir",
			self._option_entries_dir()
		).await
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub async fn checked_option_dir(&self) -> Result<String> {
		check_dir(
			"option dir",
			self._option_dir()
		).await
	}

	#[inline]
	pub async fn checked_option_manifest(&self) -> Result<String> {
		check_file(
			"option manifest",
			self._option_manifest()
		).await
	}

	#[inline]
	pub async fn checked_version_entries_dir(&self) -> Result<String> {
		check_dir(
			"version entries dir",
			self._version_entries_dir()
		).await
	}
}

impl<'h> WithVersionID<'h> {
	#[inline]
	pub async fn checked_version_dir(&self) -> Result<String> {
		check_dir(
			"version dir",
			self._version_dir()
		).await
	}

	#[inline]
	pub async fn checked_version_manifest(&self) -> Result<String> {
		check_file(
			"version manifest",
			self._version_manifest()
		).await
	}
}

pub enum SilentFailingPath {
	Ok(String),
	SilentFail,
	Err(Error)
}

pub struct SilentFailingPathResidual {
	error: Option<Error>
}

impl Try for SilentFailingPath {
	type Output = String;
	type Residual = SilentFailingPathResidual;

	#[inline]
	fn from_output(output: Self::Output) -> Self {
		Self::Ok(output)
	}

	#[inline]
	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
		use SilentFailingPath::*;
		match self {
			Ok(p) => { Continue(p) }
			SilentFail => {
				let error = None;
				Break(SilentFailingPathResidual { error })
			}
			Err(error) => {
				let error = Some(error);
				Break(SilentFailingPathResidual { error })
			}
		}
	}
}

impl FromResidual for SilentFailingPath {
	#[inline]
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		match residual.error {
			Some(e) => { SilentFailingPath::Err(e) }
			None => { SilentFailingPath::SilentFail }
		}
	}
}

impl FromResidual<SilentFailingPathResidual> for Result<Option<String>> {
	#[inline]
	fn from_residual(residual: SilentFailingPathResidual) -> Self {
		match residual.error {
			Some(e) => { Err(e) }
			None => { Ok(None) }
		}
	}
}
