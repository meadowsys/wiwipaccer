use super::error::*;
use super::fs;
use ::camino::Utf8PathBuf;
use ::std::convert::Infallible;
use ::std::future::Future;
use ::std::ffi::OsStr;
use ::std::ops::ControlFlow::{ self, Break, Continue };
use ::std::ops::{ Deref, FromResidual, Try };
use ::std::result::Result as StdResult;

// -- consts --

const ROOT_MANIFEST: &str = "pack.wiwimeta";

const TEXTURE_ENTRIES_DIR: &str = "textures";
const TEXTURE_MANIFEST: &str = "texture.wiwimeta";

const OPTION_MANIFEST: &str = "option.wiwimeta";

const OPTION_PROVIDER_MANIFEST: &str = "provider.wiwimeta";

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
pub struct WithOptionProviderID<'h> {
	with_option_id: WithOptionID<'h>,
	option_provider_id: &'h str
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

	#[inline]
	pub fn with_root_dir_osstr(self, root_dir: &OsStr) -> Result<WithRootDir> {
		Ok(self.with_root_dir(osstr_to_str(root_dir)?))
	}
}

impl<'h> WithRootDir<'h> {
	#[inline]
	pub fn with_texture_id(self, texture_id: &'h str) -> WithTextureID {
		WithTextureID { with_root_dir: self, texture_id }
	}

	#[inline]
	pub fn with_texture_id_osstr(self, texture_id: &'h OsStr) -> Result<WithTextureID> {
		Ok(self.with_texture_id(osstr_to_str(texture_id)?))
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub fn with_option_id(self, option_id: &'h str) -> WithOptionID {
		WithOptionID { with_texture_id: self, option_id }
	}

	#[inline]
	pub fn with_option_id_osstr(self, option_id: &'h OsStr) -> Result<WithOptionID> {
		Ok(self.with_option_id(osstr_to_str(option_id)?))
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub fn with_option_provider_id(self, option_provider_id: &'h str) -> WithOptionProviderID {
		WithOptionProviderID { with_option_id: self, option_provider_id }
	}

	#[inline]
	pub fn with_option_provider_id_osstr(self, option_provider_id: &'h OsStr) -> Result<WithOptionProviderID> {
		Ok(self.with_option_provider_id(osstr_to_str(option_provider_id)?))
	}
}

// -- inner getter fns --

impl<'h> WithRootDir<'h> {
	#[inline]
	pub fn root_dir_ref(&self) -> &str {
		self.root_dir
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub fn texture_id_ref(&self) -> &str {
		self.texture_id
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub fn option_id_ref(&self) -> &str {
		self.option_id
	}
}

impl<'h> WithOptionProviderID<'h> {
	#[inline]
	pub fn option_provider_id_ref(&self) -> &str {
		self.option_provider_id
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

impl<'h> Deref for WithOptionProviderID<'h> {
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
	fn _option_provider_entries_dir(&self) -> Utf8PathBuf {
		self._option_dir()
	}
}

impl<'h> WithOptionProviderID<'h> {
	#[inline]
	fn _option_provider_dir(&self) -> Utf8PathBuf {
		let mut path = self._option_dir();
		path.push(self.option_provider_id);
		path
	}

	#[inline]
	fn _option_provider_manifest(&self) -> Utf8PathBuf {
		let mut path = self._option_provider_dir();
		path.push(OPTION_PROVIDER_MANIFEST);
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

#[inline]
pub fn osstr_to_str(s: &OsStr) -> Result<&str> {
	s.to_str().ok_or_else(|| Error::NonUtf8Path)
}

// -- public interface --

impl<'h> WithRootDir<'h> {
	#[inline]
	pub async fn root_dir_checked(&self) -> Result<String> {
		check_dir(
			"root dir",
			self._root_dir()
		).await
	}

	#[inline]
	pub async fn root_manifest_checked(&self) -> Result<String> {
		check_file(
			"root manifest",
			self._root_manifest()
		).await
	}

	#[inline]
	pub async fn texture_entries_dir_checked(&self) -> Result<String> {
		check_dir(
			"texture entries dir",
			self._texture_entries_dir()
		).await
	}
}

impl<'h> WithTextureID<'h> {
	#[inline]
	pub async fn texture_dir_silent_fail(&self) -> SilentFailingPath {
		check_dir_silent_fail(
			"texture dir",
			self._texture_dir()
		).await
	}

	#[inline]
	pub async fn texture_manifest_silent_fail(&self) -> SilentFailingPath {
		check_file_silent_fail(
			"texture manifest",
			self._texture_manifest()
		).await
	}

	#[inline]
	pub async fn option_entries_dir_checked(&self) -> Result<String> {
		check_dir(
			"option entries dir",
			self._option_entries_dir()
		).await
	}
}

impl<'h> WithOptionID<'h> {
	#[inline]
	pub async fn option_dir_silent_fail(&self) -> SilentFailingPath {
		check_dir_silent_fail(
			"option dir",
			self._option_dir()
		).await
	}

	#[inline]
	pub async fn option_manifest_silent_fail(&self) -> SilentFailingPath {
		check_file_silent_fail(
			"option manifest",
			self._option_manifest()
		).await
	}

	#[inline]
	pub async fn option_provider_entries_dir_checked(&self) -> Result<String> {
		check_dir(
			"option provider entries dir",
			self._option_provider_entries_dir()
		).await
	}
}

impl<'h> WithOptionProviderID<'h> {
	#[inline]
	pub async fn option_provider_dir_silent_fail(&self) -> SilentFailingPath {
		check_dir_silent_fail(
			"option provider dir",
			self._option_provider_dir()
		).await
	}

	#[inline]
	pub async fn option_provider_manifest_silent_fail(&self) -> SilentFailingPath {
		check_file_silent_fail(
			"option provider manifest",
			self._option_provider_manifest()
		).await
	}
}

// -- SilentFailingPath custom result type --

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

impl<T, E> FromResidual<SilentFailingPathResidual> for StdResult<Option<T>, E>
where
	Error: Into<E>
{
	#[inline]
	fn from_residual(residual: SilentFailingPathResidual) -> Self {
		match residual.error {
			Some(e) => { Err(e.into()) }
			None => { Ok(None) }
		}
	}
}

impl FromResidual<StdResult<Infallible, Error>> for SilentFailingPathResidual {
	#[inline]
	fn from_residual(residual: StdResult<Infallible, Error>) -> Self {
		// hmmmmmm... residual.into_err() when
		Self { error: Some(unsafe { residual.unwrap_err_unchecked() }) }
	}
}
