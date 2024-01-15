pub struct MCVersion {
	inner: Inner
}

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

pub enum ReleaseType {
	Snapshot,
	Release,
	OldBeta,
	OldAlpha
}

pub enum PackFormat {
	Verified(u8),
	Unverified(u8),
	Unknown,
	None
}

impl ::std::ops::Deref for MCVersion {
	type Target = Inner;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

::mc_versions_macro::inject_generated_mc_versions!();
