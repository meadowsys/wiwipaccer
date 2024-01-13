pub mod error;

use crate::nom as n;
use crate::util::{ fs, path_builder, ron };
use error::*;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		versions: Vec<PackVersionSpecifier>
	}
}

#[derive(Debug, Deserialize, Serialize)]
enum PackVersionSpecifier {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}

#[derive(Debug)]
pub struct Version {
	versions: Vec<PackVersionSpecifier>,
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
