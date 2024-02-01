pub mod error;

pub use self::error::*;
use ::serde::{ Deserialize, Deserializer, Serialize, Serializer };
use ::std::mem;
use ::std::result::Result as StdResult;

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
	///
	/// Because this number is generated incrementing and creation outside this
	/// module is impossible (and we don't create any manually in here), this can
	/// be relied on for a unique value.
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
	pub fn get(version: &str) -> Result<MCVersionRef> {
		MC_VERSIONS
			.iter()
			.find(|v| v.name == version)
			.ok_or_else(|| Error::UnknownMCVersion(version.into()))
	}

	pub fn get_range(v_from: &str, v_to: &str) -> Result<MCVersionRefSlice> {
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

impl PartialEq for MCVersion {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		// this is only okay because of how the MC_VERSIONS const is
		// generated, (see doc comment for Inner::n)
		self.n == other.n
	}
}

impl Eq for MCVersion {}

impl Serialize for MCVersion {
	#[inline]
	fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
	where
		S: Serializer
	{
		<Inner as Serialize>::serialize(&self.inner, serializer)
	}
}

impl Deserialize<'static> for MCVersion {
	#[inline]
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
