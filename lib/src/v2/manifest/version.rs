use crate::v2::Map;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
enum Version {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
struct V1 {
	versions: Vec<VersionSpecifier>,
	other_versions: Map<String, Vec<OtherVersionSpecifier>>
}

#[derive(Deserialize, Serialize)]
enum VersionSpecifier {
	/// specifies a specific pack version
	PackVersion(u8),
	/// specifies a specific minecraft version
	MCVersion(String),

	/// specifies a range (inclusive) of pack versions to support
	PackVersionRange(u8, u8),
	/// specifies a range (inclusive) of minecraft versions to support. the
	/// versions are sorted by date of release. every release released between
	/// these two versions with a known pack version will be included
	MCVersionRange(String, String),

	/// specifies a pack version, and every version after it. warning: if mojang
	/// ever releases a version where they change what you rely on, then things can/will
	/// break
	PackVersionMin(u8),
	/// specifies a minecraft version, and every version after it, warning:
	/// if mojang ever releases a version where they change what you rely on, then things
	/// can/will break
	MCVersionMin(String),

	/// specifies a pack version, and every pack version less than it. Will not select
	/// minecraft versions that are marked as unknown or none
	PackVersionMax(u8),
	/// specifies a minecraft version, and every version released before it
	/// that has a pack version specified
	MCVersionMax(String)
}

#[derive(Deserialize, Serialize)]
enum OtherVersionSpecifier {
	Version(String)
}
