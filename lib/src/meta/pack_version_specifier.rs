use crate::error::{ Error, Result };
use crate::util::RON;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug)]
pub struct PackVersion {
	name: &'static str,
	release_type: MCVersionType,
	format: PackFormat
}

#[derive(Clone, Debug)]
pub enum PackFormat {
	Verified(u8),
	Unverified(u8),
	Maybe(u8),
	Unknown,
	None
}

const PACK_FORMATS: &[PackVersion] = include!("./pack_formats");

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
