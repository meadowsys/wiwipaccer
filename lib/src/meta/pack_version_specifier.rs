use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use serde::{ Deserialize, Serialize };
use super::pack_formats::PACK_FORMATS;

// TODO, try panicking, see how the frontend reacts (to see what the user will see when the expect calls fail)

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

impl PackVersionSpecifier {
	/// Returns whether specified data version (pulled from a option version) satisfies the current
	/// verion choice by the user
	pub fn contains(&self, runtime_specifier: &PackVersionSpecifierRuntimeMeta) -> bool {
		use PackVersionSpecifierRuntimeMeta::*;

		#[allow(unused)]
		match self {
			PackVersionSpecifier::MCVersion(data_version) => match runtime_specifier {
				// easy
				MCVersion(user_specified_version) => { data_version == user_specified_version }
				// is data version the only version in user specified version and matches?
				// TODO is this too restrictive? requiring that the data declare it supports the entirety of this pack version
				PackVersion(user_specified_version) => {
					PACK_FORMATS.iter().filter(|v| match v.format {
						PackFormat::None | PackFormat::Unknown => { false }
						PackFormat::Maybe(v) | PackFormat::Unverified(v) | PackFormat::Verified(v) => { &v == user_specified_version }
					}).collect::<Vec<_>>().len() == 1
				}
			}
			PackVersionSpecifier::MCVersionRange(data_version_upper, data_version_lower) => match runtime_specifier {
				// does user specified version fall into the range?
				MCVersion(user_specified_version) => {
					let index_upper = PACK_FORMATS.iter().position(|v| v.name == data_version_upper)
						.expect("critical error: index_upper, PackVersionSpecifier::MCVersionRange, PackVersionSpecifierRuntimeMeta::MCVersion");
					let index_lower = PACK_FORMATS.iter().position(|v| v.name == data_version_lower)
						.expect("critical error: index_lower, PackVersionSpecifier::MCVersionRange, PackVersionSpecifierRuntimeMeta::MCVersion");
					let index_user = PACK_FORMATS.iter().position(|v| v.name == user_specified_version)
						.expect("critical error: index_user, PackVersionSpecifier::MCVersionRange, PackVersionSpecifierRuntimeMeta::MCVersion");

					let (index_upper, index_lower) = if index_upper > index_lower {
						(index_upper, index_lower)
					} else {
						(index_lower, index_upper)
					};

					(index_lower..=index_upper).contains(&index_user)
				}
				// do all mc versions of the user specified pack version fall under
				// the data mc version range?
				// TODO is this too restrictive too? requiring that the data declare it supports the entirety of this pack version
				PackVersion(user_specified_version) => {
					let index_upper = PACK_FORMATS.iter().position(|v| v.name == data_version_upper)
						.expect("critical error: index_upper, PackVersionSpecifier::MCVersionRange, PackVersionSpecifierRuntimeMeta::PackVersion");
					let index_lower = PACK_FORMATS.iter().position(|v| v.name == data_version_lower)
						.expect("critical error: index_lower, PackVersionSpecifier::MCVersionRange, PackVersionSpecifierRuntimeMeta::PackVersion");

					let versions_with_format = PACK_FORMATS.iter().filter(|v| match v.format {
						PackFormat::None | PackFormat::Unknown => { false }
						PackFormat::Maybe(v) | PackFormat::Unverified(v) | PackFormat::Verified(v) => { &v == user_specified_version }
					}).collect::<Vec<_>>();
					let (index_upper, index_lower) = if index_upper > index_lower {
						(index_upper, index_lower)
					} else {
						(index_lower, index_upper)
					};

					let versions_specified_by_range = PACK_FORMATS.iter()
						.skip(index_lower)
						.take(index_upper)
						.collect::<Vec<_>>();

					versions_with_format.iter()
						.all(|v1| versions_specified_by_range.iter().any(|v2| v1.name == v2.name))
				}
			}
			PackVersionSpecifier::PackVersion(data_version) => match runtime_specifier {
				// does user specified version have data version?
				MCVersion(user_specified_version) => {
					let format = &PACK_FORMATS.iter()
						.find(|v| v.name == user_specified_version)
						.expect("critical error: format, PackVersionSpecifier::PackVersion, PackVersionSpecifierRuntimeMeta::MCVersion")
						.format;

					match format {
						PackFormat::None | PackFormat::Unknown => { false }
						PackFormat::Maybe(v) | PackFormat::Unverified(v) | PackFormat::Verified(v) => { v == data_version }
					}
				}
				// easy
				PackVersion(user_specified_version) => { data_version == user_specified_version }
			}
		}
	}
}

#[cfg(test)]
mod tests {
	// function naming: test_<data specifier type>_<version specifier type>
	use super::*;

	#[test]
	fn test_mcversion_packversion() {
		// this one should kinda always fail, unless there is a pack version that only one mc version uses it?
		use PackVersionSpecifier::MCVersion as DMCVersion;
		use PackVersionSpecifierRuntimeMeta::PackVersion as RPackVersion;

		assert!(!DMCVersion("1.18.2".into()).contains(&RPackVersion(8)));
		assert!(!DMCVersion("1.19.3".into()).contains(&RPackVersion(12)));
	}

	#[test]
	fn test_mcversionrange_mcversion() {
		use PackVersionSpecifier::MCVersionRange as DMCVersionRange;
		use PackVersionSpecifierRuntimeMeta::MCVersion as RMCVersion;

		assert!(DMCVersionRange("1.18".into(), "1.19".into()).contains(&RMCVersion("1.18.2".into())));
		assert!(!DMCVersionRange("1.19".into(), "23w07a".into()).contains(&RMCVersion("1.18.2".into())));
		assert!(!DMCVersionRange("22w11a".into(), "23w07a".into()).contains(&RMCVersion("1.18.2".into())));
		assert!(DMCVersionRange("1.18.2".into(), "23w07a".into()).contains(&RMCVersion("1.18.2".into())));
	}

	#[test]
	fn test_mcversionrange_packversion() {
		use PackVersionSpecifier::MCVersionRange as DMCVersionRange;
		use PackVersionSpecifierRuntimeMeta::PackVersion as RPackVersion;

		assert!(DMCVersionRange("1.18".into(), "1.19.3".into()).contains(&RPackVersion(8)));
		assert!(!DMCVersionRange("1.18".into(), "1.18.1".into()).contains(&RPackVersion(8)));
	}

	#[test]
	fn test_packversion_mcversion() {
		use PackVersionSpecifier::PackVersion as DPackVersion;
		use PackVersionSpecifierRuntimeMeta::MCVersion as RMCVersion;

		assert!(DPackVersion(12).contains(&RMCVersion("22w45a".into())));
		assert!(!DPackVersion(12).contains(&RMCVersion("22w44a".into())));
		assert!(DPackVersion(5).contains(&RMCVersion("20w14a".into())));
	}
}
