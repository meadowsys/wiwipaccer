pub mod error;

use ::serde::{ Serialize, Serializer };
use ::std::mem;
use ::std::result::Result as StdResult;
use ::std::str::FromStr;

pub struct MCVersion {
	inner: Inner
}

#[derive(Serialize)]
pub struct Inner {
	pub name: &'static str,
	pub release_type: ReleaseType,
	pub pack_format: PackFormat,

	/// release sequence ordering
	///
	/// eg. this release is the n'th release ever released\
	/// eg. "1.18.2 is the 12th release" (do not rely on this its just an example)
	///
	/// Because this number is generated incrementing and creation outside this
	/// module is impossible (and we don't create any manually in this module),
	/// this can be relied on for a unique incrementing value.
	pub n: usize
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub enum PackFormat {
	#[serde(rename = "verified")]
	Verified(u8),
	#[serde(rename = "unverified")]
	Unverified(u8),
	#[serde(rename = "none")]
	None,
	#[serde(rename = "unknown")]
	Unknown,
}

impl MCVersion {
	#[inline]
	pub fn get(version: &str)
		-> Result<MCVersionRef, error::InGetting>
	{
		version.parse()
	}



	pub fn get_range(from: &str, to: &str)
		-> Result<MCVersionRefSlice, error::InGettingRange>
	{
		let mut v_from = MC_VERSIONS
			.iter()
			.position(|v| v.name == from)
			.ok_or_else(|| error::in_getting_range_from(from, to))?;
		let mut v_to = MC_VERSIONS
			.iter()
			.position(|v| v.name == to)
			.ok_or_else(|| error::in_getting_range_to(from, to))?;

		if v_from > v_to {
			mem::swap(&mut v_from, &mut v_to);
		}

		Ok(&MC_VERSIONS[v_from..=v_to])
	}
}

impl ::std::ops::Deref for MCVersion {
	type Target = Inner;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl FromStr for MCVersionRef {
	type Err = error::InGetting;
	#[inline]
	fn from_str(s: &str) -> StdResult<Self, Self::Err> {
		MC_VERSIONS
			.iter()
			.find(|v| v.name == s)
			.ok_or_else(|| error::in_getting(s))
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

impl PackFormat {
	#[inline]
	pub fn get_version(&self) -> Option<u8> {
		use PackFormat::*;
		match self {
			Verified(v) | Unverified(v) => { Some(*v) }
			Unknown | None => { Option::None }
		}
	}
}

pub type MCVersionRef = &'static MCVersion;
pub type MCVersionRefSlice = &'static [MCVersion];

::mc_versions_macro::inject_generated_mc_versions!();
