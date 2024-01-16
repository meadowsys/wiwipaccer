pub mod error;

use crate::mc_versions::{ MC_VERSIONS, MCVersion };
use crate::nom as n;
use crate::util::{ fs, path_builder, ron };
use error::*;
use ::serde::{ Deserialize, Serialize };
use ::std::mem;

#[derive(Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		versions: Vec<PackVersionSpec>
	}
}

#[derive(Debug, Deserialize, Serialize)]
enum PackVersionSpec {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}

#[derive(Debug)]
pub struct Version {
	versions: Vec<PackVersionSpec>,
	texture_id: n::texture::ID,
	option_id: n::option::ID,
	version_id: n::version::ID,
	root_dir: n::global::RootDirPath
}

impl Version {
	pub(crate) async fn new(
		root_dir: n::global::RootDirPath,
		texture_id: n::texture::ID,
		option_id: n::option::ID,
		version_id: n::version::ID
	) -> Result<Option<Self>> {
		let p = path_builder(&root_dir)
			.with_texture(&texture_id)
			.with_option(&option_id)
			.with_version(&version_id);

		let version_dir = match p.version_dir().await {
			Ok(p) => { p }
			Err(e) if e.is_wrong_type_error() => { return Ok(None) }
			Err(e) => { return Err(e.into()) }
		};

		let meta_path = match p.version_manifest().await {
			Ok(p) => { p }
			Err(e) if e.is_wrong_type_error() => { return Ok(None) }
			Err(e) => { return Err(e.into()) }
		};

		let meta_file = fs::read_to_string(n::global::FilePath::new(meta_path.into_inner()))
			.await?;

		let meta_file = ron::from_str(&meta_file)?;

		let versions = match meta_file {
			MetaFile::Version1 { versions } => {
				versions
			}
		};

		Ok(Some(Self { versions, texture_id, option_id, version_id, root_dir }))
	}
}

impl PackVersionSpec {
	pub fn satisfies(&self, mc_version: &'static MCVersion) -> Result<bool> {
		match self {
			PackVersionSpec::PackVersion(s) => {
				let res = mc_version.pack_format
					.get_version()
					.map(|v| v == *s)
					.unwrap_or_else(|| false);
				Ok(res)
			}

			PackVersionSpec::MCVersionRange(s_from, s_to) => {
				let mut s_from = MC_VERSIONS.iter()
					.find(|v| v.name == s_from)
					.ok_or_else(|| Error::UnknownMCVersions(s_from.into()))?;

				let mut s_to = MC_VERSIONS.iter()
					.find(|v| v.name == s_to)
					.ok_or_else(|| Error::UnknownMCVersions(s_to.into()))?;

				if s_from.n > s_to.n {
					mem::swap(&mut s_from, &mut s_to);
				}

				Ok(s_from.n <= mc_version.n && mc_version.n <= s_to.n)
			}

			PackVersionSpec::MCVersion(s) => {
				let s = MC_VERSIONS.iter()
					.find(|v| v.name == s)
					.ok_or_else(|| Error::UnknownMCVersions(s.into()))?;

				Ok(s.n == mc_version.n)
			}
		}
	}
}

pub struct FrontendData<'h> {
	versions: &'h [PackVersionSpec],
	version_id: &'h n::version::ID
}

impl<'h> FrontendData<'h> {
	pub fn new(version: &'h Version) -> Self {
		let Version { versions, version_id, .. } = version;
		Self { versions, version_id }
	}
}
