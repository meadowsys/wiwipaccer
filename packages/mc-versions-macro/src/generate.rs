use ::proc_macro2::TokenStream;
use ::quote::quote;
use ::serde::{ Deserialize, Serialize };

const VERSION_MANIFEST_V2: &str = include_str!("./version_manifest_v2.json");

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Manifest {
	latest: Latest,
	versions: Vec<Version>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Latest {
	release: String,
	snapshot: String
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Version {
	id: String,
	r#type: String,
	url: String,
	time: String,
	#[serde(rename = "releaseTime")]
	release_time: String,
	sha1: String,
	#[serde(rename = "complianceLevel")]
	compliance_level: u8
}

pub(crate) fn inject_generated_mc_versions(input: TokenStream) -> TokenStream {
	match inject_generated_mc_versions_inner(input) {
		Ok(t) | Err(t) => { t }
	}
}

fn inject_generated_mc_versions_inner(_: TokenStream) -> Result<TokenStream, TokenStream> {
	let manifest = ::serde_json::from_str::<Manifest>(VERSION_MANIFEST_V2)
		.map_err(|err| {
			let message = format!("parsing manifest had an error: {err}");
			quote! {
				compiler_error!(#message);
			}
		})?;

	// validation of release/snapshot (not necessary but ueah)
	let release = manifest.versions.iter().any(|v| v.id == manifest.latest.release);
	let snapshot = manifest.versions.iter().any(|v| v.id == manifest.latest.snapshot);

	match (release, snapshot) {
		(true, true) => { Ok(()) }
		(true, false) => { Err(quote! {
			compiler_error!("manifest is invalid: latest snapshot does not have corresponding entry in versions");
		}) }
		(false, true) => { Err(quote! {
			compiler_error!("manifest is invalid: latest release does not have corresponding entry in versions");
		}) }
		(false, false) => { Err(quote! {
			compiler_error!("manifest is invalid: both latest release and snapshot do not have corresponding entries in versions");
		}) }
	}?;

	let Manifest { latest, versions } = manifest;

	let mut versions = versions.into_iter()
		.map(|v| {
			(::chrono::DateTime::parse_from_rfc3339(&v.release_time).unwrap(), v)
		})
		.collect::<Vec<_>>();
	versions.sort_unstable_by_key(|v| std::cmp::Reverse(v.0));

	let versions = versions.into_iter()
		.map(|(_, Version { id: name, r#type, .. })| {
			let release_type = match &*r#type {
				"snapshot" => { quote! { ReleaseType::Snapshot } }
				"release" => { quote! { ReleaseType::Release } }
				"old_beta" => { quote! { ReleaseType::OldBeta } }
				"old_alpha" => { quote! { ReleaseType::OldAlpha } }
				t => { unreachable!("unexpectedly got \"{t}\" for a release type") }
			};

			// TODO solve this
			let pack_format = quote! { PackFormat::Unknown };

			quote! {
				MCVersion {
					name: #name,
					release_type: #release_type,
					pack_format: #pack_format
				}
			}
		})
		.collect::<Vec<_>>();

	Ok(quote! {
		pub mod mc_versions {
			pub struct MCVersion {
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

			pub const MC_VERSIONS: &[MCVersion] = &[
				#( #versions ),*
			];
		}
	})
}
