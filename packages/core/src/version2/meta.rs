use ::serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
#[serde(tag = "meta_version")]
pub(super) enum VersionMeta {
	#[serde(rename = "1")]
	Version1 {
		versions: Vec<PackVersionSpecMeta>
	}
}

#[derive(Deserialize, Serialize)]
pub(super) enum PackVersionSpecMeta {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}
