use super::*;

#[derive(Deserialize, Serialize)]
pub enum Version {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	pub versions: Vec<VersionSpecifier>,
	pub other_versions: Map<String, Vec<OtherVersionSpecifier>>
}

#[derive(Deserialize, Serialize)]
pub enum VersionSpecifier {
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
pub enum OtherVersionSpecifier {
	Version(String)
}

impl VersionSpecifier {
	pub fn contains_mc_version(&self, mc_version: &PackVersion) -> bool {
		use VersionSpecifier::*;
		match self {
			// if mc_version is equal to self
			PackVersion(version) => {
				mc_version.format
					.get_num()
					.map(|v| v == *version)
					.unwrap_or_else(|| false)
			}
			MCVersion(version) => {
				mc_version.name == version
			}

			// if mc_version is in the range of self
			PackVersionRange(min, max) => {
				mc_version.format
					.get_num()
					.map(|v| (*min..*max).contains(&v))
					.unwrap_or_else(|| false)
			}
			MCVersionRange(min, max) => {
				// FIXME: this can be optimised
				// add a lazily initialised static map mapping version names to their PackVersion
				// references
				// *maybe* add a field to PackVersion that contains statically its index? hmm i dunno
				let (mut min_i, mut max_i, mut mc_version_i) = (None, None, None);

				// unnecessary optimisation: iterate through once, checking all of them at the same time
				// instead of .iter().find() which will iterate through up to 3 times
				for (i, v) in PACK_FORMATS.iter().enumerate() {
					// match v.name {
					// 	min => {
					// 		if min_i.is_some() { panic!() }
					// 	}
					// }

					if (v.name == min) {
						if min_i.is_some() { panic!("potential dupe version with name {min} found") }
						min_i = Some(i);
					}
					if (v.name == max) {
						if max_i.is_some() { panic!("potential dupe version with name {max} found") }
						max_i = Some(i);
					}
					if (v.name == mc_version.name) {
						if mc_version_i.is_some() { panic!("potential dupe version with name {} found", mc_version.name) }
						mc_version_i = Some(i);
					}

					// in dev, don't exit loop early, because iterating through
					// the entire thing may help find dupes
					#[cfg(not(debug_assertions))]
					if min_i.is_some() && max_i.is_some() && mc_version_i.is_some() { break }
				}

				if let Some(min_i) = min_i {
					if let Some(max_i) = max_i {
						if let Some(mc_version_i) = mc_version_i {
							return min_i <= mc_version_i && mc_version_i <= max_i
						}
					}
				}

				false
			}

			// self is the min, so if mc_version is greater than self
			PackVersionMin(min) => {
				mc_version.format
					.get_num()
					.map(|v| min < &v)
					.unwrap_or_else(|| false)
			}
			MCVersionMin(min) => {
				let (mut min_i, mut mc_version_i) = (None, None);

				for (i, v) in PACK_FORMATS.iter().enumerate() {
					if v.name == min {
						if min_i.is_some() { panic!("potential dupe version with name {min} found") }
						min_i = Some(i);
					}
					if v.name == mc_version.name {
						if mc_version_i.is_some() { panic!("potential dupe version with name {} found", mc_version.name) }
						mc_version_i = Some(i);
					}

					// in dev, don't exit loop early, because iterating through
					// the entire thing may help find dupes
					#[cfg(not(debug_assertions))]
					if min_i.is_some() && mc_version_i.is_some() { break }
				}

				if let Some(min_i) = min_i {
					if let Some(mc_version_i) = mc_version_i {
						return min_i <= mc_version_i
					}
				}

				false
			}

			// self is the max, so if mc_version is less than self
			PackVersionMax(max) => {
				mc_version.format
					.get_num()
					.map(|v| max > &v)
					.unwrap_or_else(|| false)
			}
			MCVersionMax(max) => {
				let (mut mc_version_i, mut max_i) = (None, None);

				for (i, v) in PACK_FORMATS.iter().enumerate() {
					if v.name == max {
						if max_i.is_some() { panic!("potential dupe version with name {max} found") }
						max_i = Some(i);
					}
					if v.name == mc_version.name {
						if mc_version_i.is_some() { panic!("potential dupe version with name {} found", mc_version.name) }
						mc_version_i = Some(i);
					}

					// in dev, don't exit loop early, because iterating through
					// the entire thing may help find dupes
					#[cfg(not(debug_assertions))]
					if max_i.is_some() && mc_version_i.is_some() { break }
				}

				if let Some(mc_version_i) = mc_version_i {
					if let Some(max_i) = max_i {
						return mc_version_i <= max_i
					}
				}

				false
			}
		}
	}
}
