use chrono::DateTime;
use chrono::offset::Utc as UTC;
use crate::error::{ Error, Result };
use crate::util::RON;
use serde::{ Deserialize, Serialize };

pub const MC_VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Debug, Deserialize, Serialize)]
pub enum PackVersionSpecifier {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}

impl PackVersionSpecifier {
	pub async fn fetch_mc_versions() -> Result<MCReleases> {
		let client = reqwest::ClientBuilder::new()
			.brotli(true)
			.gzip(true)
			.https_only(true)
			.no_trust_dns()
			.build()
			.map_err(|e| Error::UnableToInitialiseHttpClient { source: e })?;

		let fetched = client.get(MC_VERSION_MANIFEST_URL)
			.send()
			.await
			.map_err(|e| Error::FailedToFetchMCVersions { source: e })?
			.bytes()
			.await
			.map_err(|e| Error::FailedToFetchMCVersions { source: e })?
			.into_iter()
			.collect::<Vec<_>>();
		let fetched = String::from_utf8(fetched)
			.map_err(|e| Error::FailedToFetchMCVersionsInvalidUTF8 { source: e })?;

		let parsed = RON.from_str::<MCReleases>(&fetched)
			.map_err(|e| Error::ParseErrorRonSpannedError { path: "<http response>".into(), source: e })?;

		Ok(parsed)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MCReleases {
	latest: MCLatestRelease,
	versions: Vec<MCVersion>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MCLatestRelease {
	release: String,
	snapshot: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MCVersion {
	id: String,
	#[serde(rename = "type")]
	version_type: MCVersionType,
	#[serde(rename = "url")]
	manifest_url: String,
	#[serde(rename = "time")]
	time_generated_by_server_or_something: DateTime<UTC>,
	#[serde(rename = "releaseTime")]
	release_time: DateTime<UTC>
}

#[derive(Debug, Deserialize, Serialize)]
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
