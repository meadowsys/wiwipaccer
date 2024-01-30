use crate::gen::Generator;
use crate::mc_versions::{ MC_VERSIONS, MCVersionRef, MCVersionRefSlice };
use crate::util::fs;
use crate::util::path_builder3::WithProviderID;
use super::error::*;
use super::{ meta, nr };

pub struct ProviderRuntime {
	id: nr::ID,
	gen: Generator
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

impl ProviderRuntime {
	pub(crate) async fn new(p: &WithProviderID<'_>) -> Result<Option<Self>> {
		let dir = p.provider_dir_silent_fail().await?;
		let meta_path = p.provider_manifest_silent_fail().await?;
		let meta_file = fs::read_to_string2(meta_path).await?;
		let meta::ProviderUnversioned {
			gen
		} = meta::deserialise_version(&meta_file)?;

		let id = nr::ID::new(p.provider_id_ref().into());

		Ok(Some(Self {
			id,
			gen
		}))
	}
}
