//! root manifest for pack sources (equivalent-ish to pack.mcmeta of a resource pack,
//! i suppose)

use camino::{ Utf8Path, Utf8PathBuf };
use crate::error::{ self, Error, Result };
use crate::ron;
use crate::settings::Settings;
use crate::texture::{ NewTextureOptions, Texture, TEXTURES_DIR };
use crate::util;
use serde::{ Deserialize, Serialize };
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub const SOURCE_META_FILENAME: &str = "pack.wiwimeta";

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		pack_id: String,
		description: Option<String>,
		version: Option<String>
	}
}

#[derive(Debug)]
pub struct Source {
	name: String,
	dir: Utf8PathBuf,
	pack_id: String,
	description: Option<String>,
	version: String,
	textures: Vec<Texture>
}

impl Source {
	pub async fn new(dir: Utf8PathBuf, settings: &Settings) -> Result<Self> {
		if !util::check_is_dir(&dir).await? {
			return Err(Error::PackSourcePathIsNotDir)
		}

		let mut manifest_path = dir.clone();
		manifest_path.push(SOURCE_META_FILENAME);

		if !util::check_is_file(&manifest_path).await? {
			return Err(Error::PackSourceDirContainsNoManifest)
		}

		let mut manifest_reader = fs::OpenOptions::new()
			.read(true)
			.open(&manifest_path)
			.await
			.map_err(error::file_io_error(&manifest_path))?;

		let manifest_meta = fs::metadata(&manifest_path)
			.await
			.map_err(error::file_io_error(&manifest_path))?;
		let mut manifest_file = Vec::with_capacity(manifest_meta.len() as usize);

		manifest_reader.read_to_end(&mut manifest_file)
			.await
			.map_err(error::file_io_error(&manifest_path))?;

		let manifest_file = String::from_utf8(manifest_file)?;

		let (name, pack_id, description, version) = match ron::from_str(&manifest_file)? {
			MetaFile::Version1 { name, pack_id, description, version } => {
				(name, pack_id, description, version)
			}
		};

		let version = match version {
			None => { "unknown".into() }
			Some(version) => {
				const GIT_HASH: &str = "git-hash";
				const GIT_SHORT_HASH: &str = "git-short-hash";
				const GIT_TAG: &str = "git-tag";

				match &*version {
					GIT_HASH => {
						git_rev_parse_head(&dir, &settings.git_executable).await?
					}
					GIT_SHORT_HASH => {
						let hash = git_rev_parse_head(&dir, &settings.git_executable).await?;
						hash[..10].into()
					}
					GIT_TAG => {
						git_tag(&dir, &settings.git_executable).await?
					}
					_ => { version }
				}
			}
		};

		let textures = read_textures(&dir)
			.await?;

		Ok(Source { name, dir, pack_id, description, version, textures })
	}
}

async fn git_rev_parse_head(dir: &Utf8Path, git: &str) -> Result<String> {
	const REV_PARSE_HEAD_CMD: &str = "git rev-parse HEAD";
	const REV_PARSE_HEAD_CMD_ARGS: &[&str] = &["rev-parse", "HEAD"];

	let child = Command::new(git)
		.args(REV_PARSE_HEAD_CMD_ARGS)
		.stdout(Stdio::piped())
		.current_dir(dir)
		.spawn()
		.map_err(|source| Error::ChildProcessFailedToSpawnForGitVersioning {
			source,
			command: REV_PARSE_HEAD_CMD.into()
		})?;
	let output = child.wait_with_output()
		.await
		.map_err(|source| Error::ChildProcessFailedToSpawnForGitVersioning {
			source,
			command: REV_PARSE_HEAD_CMD.into()
		})?;

	let hash = String::from_utf8(output.stdout)?
		.trim()
		.into();

	Ok(hash)
}

async fn git_tag(dir: &Utf8Path, git: &str) -> Result<String> {
	const TAG_CMD: &str = "git describe --tags --abbrev=10 --always";
	const TAG_CMD_ARGS: &[&str] = &["describe", "--tags", "--abbrev=10", "--always"];

	let child = Command::new(git)
		.args(TAG_CMD_ARGS)
		.stdout(Stdio::piped())
		.current_dir(dir)
		.spawn()
		.map_err(|source| Error::ChildProcessFailedToSpawnForGitVersioning {
			source,
			command: TAG_CMD.into()
		})?;
	let output = child.wait_with_output()
		.await
		.map_err(|source| Error::ChildProcessFailedToSpawnForGitVersioning {
			source,
			command: TAG_CMD.into()
		})?;

	let tag = String::from_utf8(output.stdout)?
		.trim()
		.into();

	Ok(tag)
}

async fn read_textures(dir: &Utf8Path) -> Result<Vec<Texture>> {
	let mut textures_dir = dir.to_owned();
	textures_dir.push(TEXTURES_DIR);
	let mut dir_contents = fs::read_dir(&textures_dir)
		.await
		.map_err(|source| Error::FileIOError { source, path: textures_dir.clone() })?;
	let mut textures = vec![];

	while let Some(entry) = {
		dir_contents
			.next_entry()
			.await
			.map_err(|source| Error::FileIOError { source, path: textures_dir.clone() })?
	} {
		let dir_name: std::path::PathBuf = entry.file_name().into();
		let dir_name = dir_name.try_into()
			.map_err(|_| Error::NonUTF8PathsUnsupported)?;

		let options = NewTextureOptions {
			root_dir: dir.into(),
			dir_name
		};

		if let Some(texture) = Texture::new(options).await? {
			textures.push(texture);
		}
	}

	Ok(textures)
}
