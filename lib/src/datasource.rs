//! root manifest for a datasource (equivalent-ish to pack.mcmeta of a resource pack,
//! i suppose)

use camino::{ Utf8Path, Utf8PathBuf };
use crate::error::*;
use serde::{ Deserialize, Serialize };
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

const SOURCE_META_FILENAME: &str = "pack.wiwimeta";

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		packid: String,
		description: Option<String>,
		version: Option<String>
	}
}

#[derive(Debug)]
pub struct Source {
	name: String,
	packid: String,
	description: Option<String>,
	version: String
}

impl Source {
	pub async fn new(dir: Utf8PathBuf) -> Result<Self> {
		let _ = fs::read_dir(&dir).await
			.map_err(|source| Error::SourceDirReadError { source, path: dir.clone() })?;

		let mut manifest_path = dir.clone();
		manifest_path.push(SOURCE_META_FILENAME);

		let mut manifest_reader = fs::OpenOptions::new()
			.read(true)
			.open(&manifest_path)
			.await
			.map_err(|source| Error::SourceManifestReadError { source, path: manifest_path.clone() })?;
		let manifest_meta = fs::metadata(&manifest_path)
			.await
			.map_err(|source| Error::FileIOError { source, path: manifest_path.clone() })?;

		let mut manifest_file = Vec::with_capacity(manifest_meta.len() as usize);
		manifest_reader.read_to_end(&mut manifest_file)
			.await
			.map_err(|source| Error::FileIOError { source, path: manifest_path })?;

		let manifest_file = String::from_utf8(manifest_file)?;

		let (name, packid, description, version) = match ron::from_str(&manifest_file)? {
			MetaFile::Version1 { name, packid, description, version } => {
				(name, packid, description, version)
			}
		};

		let version = match version {
			None => { "unknown".into() }
			Some(version) => {
				const GIT_HASH: &str = "git-hash";
				const GIT_SHORT_HASH: &str = "git-short-hash";
				const GIT_TAG: &str = "git-tag";

				// TODO: make git executable configurable (from the GUI too)
				match &*version {
					GIT_HASH => {
						git_rev_parse_head(&dir)
							.await?
					}
					GIT_SHORT_HASH => {
						let hash = git_rev_parse_head(&dir).await?;
						hash[..10].into()
					}
					GIT_TAG => {
						git_tag(&dir).await?
					}
					_ => { version }
				}
			}
		};

		Ok(Source { name, packid, description, version })
	}
}

async fn git_rev_parse_head(dir: &Utf8Path) -> Result<String> {
	const REV_PARSE_HEAD_CMD: &str = "git rev-parse HEAD";
	const REV_PARSE_HEAD_CMD_ARGS: (&str, &[&str]) = ("git", &["rev-parse", "HEAD"]);

	let child = Command::new(REV_PARSE_HEAD_CMD_ARGS.0)
		.args(REV_PARSE_HEAD_CMD_ARGS.1)
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

async fn git_tag(dir: &Utf8Path) -> Result<String> {
	const TAG_CMD: &str = "git describe --tags --abbrev=10 --always";
	const TAG_CMD_ARGS: (&str, &[&str]) = ("git", &["describe", "--tags", "--abbrev=10", "--always"]);

	let child = Command::new(TAG_CMD_ARGS.0)
		.args(TAG_CMD_ARGS.1)
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
