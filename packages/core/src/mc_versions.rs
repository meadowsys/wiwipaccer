use ::serde::{ Deserialize, Deserializer, Serialize, Serializer };

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
	#[serde(rename = "unverified")]
	Unverified(u8),
	#[serde(rename = "unknown")]
	Unknown,
	#[serde(rename = "none")]
	None
}

impl ::std::ops::Deref for MCVersion {
	type Target = Inner;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl Serialize for MCVersion {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		<Inner as Serialize>::serialize(&self.inner, serializer)
	}
}

impl Deserialize<'static> for MCVersion {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
			Verified(v) | Unverified(v) => { Some(*v) }
			Unknown | None => { Option::None }
		}
	}
}

::mc_versions_macro::inject_generated_mc_versions!();
