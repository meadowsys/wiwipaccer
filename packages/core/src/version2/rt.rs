use super::error::*;
use crate::mc_versions::{ MC_VERSIONS, MCVersionRef, MCVersionRefSlice };
use super::nr;

pub struct VersionRuntime {
	id: nr::ID,
	versions: Vec<PackVersionSpecRuntime>
}

pub enum PackVersionSpecRuntime {
	PackVersion(u8),
	MCVersion(MCVersionRef),
	MCVersionRange(MCVersionRefSlice)
}

impl PackVersionSpecRuntime {
	pub fn satisfies(&self, mc_version: MCVersionRef) -> bool {
		use PackVersionSpecRuntime::*;
		match self {
			PackVersion(s) => {
				mc_version.pack_format
					.get_version()
					.map(|v| v == *s)
					.unwrap_or_else(|| false)
			}

			MCVersion(s) => {
				**s == *mc_version
			}

			MCVersionRange(s) => {
				s.iter().any(|s| *s == *mc_version)
			}
		}
	}
}

impl VersionRuntime {
	pub(crate) async fn new() -> Result<Self> {
		todo!()
	}
}
