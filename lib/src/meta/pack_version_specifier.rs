use crate::error::{ Error, Result };
use crate::util::RON;
use serde::{ Deserialize, Serialize };
use super::pack_formats::PACK_FORMATS;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackVersion {
	pub name: &'static str,
	pub release_type: MCVersionType,
	pub format: PackFormat
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PackFormat {
	Verified(u8),
	Unverified(u8),
	Maybe(u8),
	Unknown,
	None
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PackVersionSpecifier {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MCVersionType {
	#[serde(rename = "release")]
	Release,
	#[serde(rename = "snapshot")]
	Snapshot,
	#[serde(rename = "old_beta")]
	OldBeta,
	#[serde(rename = "old_alpha")]
	OldAlpha
}
