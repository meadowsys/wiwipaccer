use ::serde::{ Deserialize, Deserializer, Serialize, Serializer };
use ::std::mem;
use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub struct MCVersion {
	inner: Inner
}

#[derive(Deserialize, Serialize)]
pub struct Inner {
	pub name: &'static str,
	pub release_type: ReleaseType,
	pub pack_format: PackFormat,

	/// release ordering
	///
	/// eg. this release is the n'th release ever released\
	/// eg. "1.18.2 is the 12th release" (do not rely on this its just an example)
	pub n: usize
}

#[derive(Deserialize, Serialize)]
pub enum ReleaseType {
	#[serde(rename = "snapshot")]
	Snapshot,
	#[serde(rename = "release")]
	Release,
	#[serde(rename = "old_beta")]
	OldBeta,
	#[serde(rename = "old_alpha")]
	OldAlpha
}

#[derive(Deserialize, Serialize)]
pub enum PackFormat {
	#[serde(rename = "verified")]
	Verified(u8),
	#[serde(rename = "none")]
	None,
	#[serde(rename = "unknown")]
	Unknown,
}

impl MCVersion {
	pub fn get_mc_version(version: &str) -> Result<MCVersionRef> {
		MC_VERSIONS
			.iter()
			.find(|v| v.name == version)
			.ok_or_else(|| Error::UnknownMCVersion(version.into()))
	}

	pub fn get_mc_version_range(v_from: &str, v_to: &str) -> Result<MCVersionRefSlice> {
		let mut v_from = MC_VERSIONS
			.iter()
			.position(|v| v.name == v_from)
			.ok_or_else(|| Error::UnknownMCVersion(v_from.into()))?;
		let mut v_to = MC_VERSIONS
			.iter()
			.position(|v| v.name == v_to)
			.ok_or_else(|| Error::UnknownMCVersion(v_to.into()))?;

		if v_from > v_to {
			mem::swap(&mut v_from, &mut v_to);
		}

		Ok(&MC_VERSIONS[v_from..=v_to])
	}
}

impl ::std::ops::Deref for MCVersion {
	type Target = Inner;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl Serialize for MCVersion {
	fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
	where
		S: Serializer
	{
		<Inner as Serialize>::serialize(&self.inner, serializer)
	}
}

impl Deserialize<'static> for MCVersion {
	fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
	where
		D: Deserializer<'static>
	{
		let inner = <Inner as Deserialize>::deserialize(deserializer)?;
		Ok(Self { inner })
	}
}

impl PackFormat {
	#[inline]
	pub fn get_version(&self) -> Option<u8> {
		use PackFormat::*;
		match self {
			Verified(v) => { Some(*v) }
			Unknown | None => { Option::None }
		}
	}
}

pub type MCVersionRef = &'static MCVersion;
pub type MCVersionRefSlice = &'static [MCVersion];

::mc_versions_macro::inject_generated_mc_versions!();

// TODO: can this be moved into a better place / done better?
pub type Result<T> = StdResult<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
	#[error("unknown minecraft version: {0}")]
	UnknownMCVersion(String)
}
