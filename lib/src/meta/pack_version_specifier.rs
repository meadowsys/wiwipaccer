use chrono::DateTime;
use chrono::offset::Utc as UTC;
use crate::error::{ Error, Result };
use crate::util::RON;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug)]
pub struct PackVersion {
	name: &'static str,
	release_type: MCVersionType,
	format: Option<u8>
}

const PACK_FORMATS: &[PackVersion] = include!("./pack_formats");

#[derive(Debug, Deserialize, Serialize)]
pub enum PackVersionSpecifier {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}

/// Responses to release list endpoint from Mojang
#[derive(Debug, Deserialize, Serialize)]
pub struct MCReleases {
	pub latest: MCLatestRelease,
	pub versions: Vec<MCVersion>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MCLatestRelease {
	pub release: String,
	pub snapshot: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MCVersion {
	pub id: String,
	#[serde(rename = "type")]
	pub version_type: MCVersionType,
	#[serde(rename = "url")]
	pub manifest_url: String,
	#[serde(rename = "time")]
	pub time_generated_by_server_or_something: DateTime<UTC>,
	#[serde(rename = "releaseTime")]
	pub release_time: DateTime<UTC>
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
