pub struct MCVersion {
	inner: Inner
}

pub struct Inner {
	pub name: &'static str,
	pub release_type: ReleaseType,
	pub pack_format: PackFormat
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
