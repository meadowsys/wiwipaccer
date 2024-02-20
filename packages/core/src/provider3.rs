use crate::error::provider_err;
use crate::gen::Generator;
use ::mc_versions::{ MC_VERSIONS, MCVersionRef, MCVersionRefSlice };
use crate::util::fs2 as fs;
use crate::util::path_builder4::WithProviderID;
use crate::util::ron2 as ron;
use ::serde::{ Deserialize, Serialize };


::nominal::nominal_mod! {
	pub mod nr {
		nominal!(pub ID, inner: String);
	}
}


mod meta {
	use super::*;

	#[derive(Serialize, Deserialize)]
	#[serde(tag = "meta_version")]
	enum ProviderMeta {
		#[serde(rename = "1")]
		Version1 {
			#[serde(flatten)]
			gen: Generator
		}
	}

	pub(super) struct ProviderUnversioned {
		pub(super) gen: Generator
	}

	pub(super) fn deserialise_provider(s: &str)
		-> Result<ProviderUnversioned, provider_err::DeserialiseMeta>
	{
		use ProviderMeta::*;
		Ok(match ron::from_str(s)? {
			Version1 { gen } => {
				ProviderUnversioned { gen }
			}
		})
	}
}


mod rt {
	use super::*;

	pub struct ProviderRuntime {
		id: nr::ID,
		gen: Generator
	}

	// pub enum PackVersionSpecRuntime {
	// 	PackVersion(u8),
	// 	MCVersion(MCVersionRef),
	// 	MCVersionRange(MCVersionRefSlice)
	// }
	//
	// impl PackVersionSpecRuntime {
	// 	pub fn satisfies(&self, mc_version: MCVersionRef) -> bool {
	// 		use PackVersionSpecRuntime::*;
	// 		match self {
	// 			PackVersion(s) => {
	// 				mc_version.pack_format
	// 					.get_version()
	// 					.map(|v| v == *s)
	// 					.unwrap_or_else(|| false)
	// 			}
	//
	// 			MCVersion(s) => {
	// 				**s == *mc_version
	// 			}
	//
	// 			MCVersionRange(s) => {
	// 				s.iter().any(|s| *s == *mc_version)
	// 			}
	// 		}
	// 	}
	// }

	impl ProviderRuntime {
		pub(crate) async fn new(p: &WithProviderID<'_>) -> Result<Option<Self>, provider_err::New> {
			let dir = p.provider_dir().fail_silently().await?;
			let meta_path = p.provider_manifest().fail_silently().await?;
			let meta_file = fs::read_to_string(|| meta_path.clone()).await
				.map_err(provider_err::reading_meta_file)?;
			let meta::ProviderUnversioned {
				gen
			} = meta::deserialise_provider(&meta_file)?;

			let id = nr::ID::new(p.provider_id_ref().into());

			Ok(Some(Self {
				id,
				gen
			}))
		}
	}

	#[derive(Serialize)]
	pub struct FrontendData<'h> {
		id: &'h nr::ID
	}

	impl<'h> FrontendData<'h> {
		pub fn new(provider: &'h ProviderRuntime, mc_version: MCVersionRef) -> Option<Self> {
			if !provider.gen.is_available_for(mc_version) { return None }

			let id = &provider.id;
			Some(Self { id })
		}
	}
}
