pub mod action;
pub mod datasource;
pub mod option;
pub mod pack_version_specifier;
pub mod texture;
pub mod version;

use crate::error::{ Error, Result };
use crate::util::RON;
use serde::de::DeserializeOwned;
use tokio::fs;

const META_NAME: &str = "manifest.wpm";
const ASSETS_DIR_NAME: &str = "assets";
const TEXTURES_DIR: &str = "textures";

#[derive(Clone, Debug, serde::Serialize)]
pub struct Message {
	pub message: String,
	pub severity: MessageSeverity
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MessageSeverity {
	Info,
	Warning,
	Error,
	Fatal
}

impl std::fmt::Display for MessageSeverity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use MessageSeverity::*;

		match self {
			Info => { write!(f, "Info") }
			Warning => { write!(f, "Warning") }
			Error => { write!(f, "Error") }
			Fatal => { write!(f, "Fatal") }
		}
	}
}

#[macro_export]
macro_rules! impl_deref {
	($type:ty, target $target:ty) => {
		impl std::ops::Deref for $type {
			type Target = $target;
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	}
}

pub async fn read_meta_file<T: DeserializeOwned>(dir: &str) -> Result<T> {
	let manifest_path = format!("{dir}/{META_NAME}");

	let manifest_file_meta = fs::metadata(&manifest_path).await
		.map_err(|e| Error::ManifestDoesNotExist { path: manifest_path.clone(), source: e })?;
	if !manifest_file_meta.is_file() {
		return Err(Error::ManifestIsNotFile { path: manifest_path })
	}

	let file = fs::read_to_string(&manifest_path).await
		.map_err(|e| Error::IOError { source: e })?;
	let parsed = RON.from_str::<T>(&file)
		.map_err(|e| Error::ParseErrorRonSpannedError {
			path: manifest_path,
			source: e
		})?;

	Ok(parsed)
}
