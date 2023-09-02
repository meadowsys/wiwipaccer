use ahash::{ RandomState, HashMapExt };
use crate::runtime_meta::datasource::{ self, Datasource };
use super::super::util::RON;
use crate::error::{ Error, Result };
use crate::external_meta::pack_formats::PackVersion;
use crate::runtime_meta::datasource::BuildType;
use super::super::util::sort_versions_inefficient;
use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, serde::Serialize)]
pub struct Workspace(InnerWorkspace);

#[derive(Debug, serde::Serialize)]
pub struct InnerWorkspace {
	pub sources: WorkspaceDatasources
}

#[derive(Debug, serde::Serialize)]
pub enum WorkspaceDatasources {
	WithoutMCVersion(WithoutMCVersion),
	WithMCVersion(WithMCVersion)
}

#[derive(Debug, serde::Serialize)]
pub struct WithoutMCVersion {
	pub sources: Vec<Datasource>
}

#[derive(Debug, serde::Serialize)]
pub struct WithMCVersion {
	pub version: String,
	pub sources: Vec<Datasource>
}

crate::impl_deref!(Workspace, target InnerWorkspace);

impl Workspace {
	#[inline]
	pub async fn single_dir(path: &str) -> Result<Self> {
		Self::multiple_dirs(&[path]).await
	}

	async fn multiple_dirs(paths: &[&str]) -> Result<Self> {
		// TODO this cannot be empty as it only is called by `single_dir` right now,
		// uncomment this when multiple dirs support is implemented
		// if paths.is_empty() { return Err(Error::MustSpecifyAtLeastOneSource) }

		let mut datasources = Vec::with_capacity(paths.len());

		for path in paths {
			datasources.push(Datasource::new(path).await?);
		}

		let mut sources = Vec::with_capacity(paths.len());

		datasources.into_iter()
			.for_each(|s| sources.push(s));

		Ok(Self(InnerWorkspace {
			sources: WorkspaceDatasources::WithoutMCVersion(WithoutMCVersion {
				sources
			})
		}))
	}

	pub fn get_names(&self) -> Vec<&str> {
		match &self.sources {
			WorkspaceDatasources::WithoutMCVersion(WithoutMCVersion { sources })
				| WorkspaceDatasources::WithMCVersion(WithMCVersion { sources, .. })
			=> {
				sources.iter()
					.map(|s| s.get_name())
					.collect()
			}
		}
	}

	pub fn get_supported_mc_versions(&self) -> Result<Vec<PackVersion>> {
		let versions_results = match &self.sources {
			WorkspaceDatasources::WithMCVersion(sources) => { &sources.sources }
			WorkspaceDatasources::WithoutMCVersion(sources) => { &sources.sources }
		};
		let versions_results = versions_results.iter()
			.map(|s| s.get_supported_mc_versions_no_sort())
			.collect::<Vec<_>>();

		let mut versions = Vec::with_capacity(versions_results.len());
		for version in versions_results {
			versions.push(version?);
		}

		let mut versions = versions.into_iter()
			.flatten()
			.collect();
		sort_versions_inefficient(&mut versions);
		Ok(versions)
	}

	pub async fn with_mc_version(self, mc_version: String) -> Self {
		let sources = match self.0.sources {
			WorkspaceDatasources::WithMCVersion(sources) => { sources.sources }
			WorkspaceDatasources::WithoutMCVersion(sources) => { sources.sources }
		};

		let mut versioned = Vec::with_capacity(sources.len());

		for source in sources {
			versioned.push(source.with_mc_version(mc_version.clone()).await);
		}

		Self(InnerWorkspace {
			sources: WorkspaceDatasources::WithMCVersion(WithMCVersion {
				version: mc_version,
				sources: versioned
			})
		})
	}

	pub async fn build(
		&self,
		dir: &str,
		choices: &Vec<HashMap<String, String, RandomState>>,
		buildtype: BuildType
	) -> Result<()> {
		let (sources, _version) = match &self.sources {
			WorkspaceDatasources::WithMCVersion(WithMCVersion { sources, version }) => { Ok((sources, version)) }
			WorkspaceDatasources::WithoutMCVersion(_) => { Err(Error::MCVersionUnspecified) }
		}?;

		if choices.len() != sources.len() {
			return Err(Error::WorkspaceChoicesLenDoesntMatch {
				choices_len: choices.len(),
				sources_len: sources.len()
			})
		}

		// TODO this could be smarter in terms of collission prevention, and
		// datasources could work together in smarter ways than this
		// but for now, not going to do multisource, so this is a future me problem lol
		for (source, choices) in sources.iter().zip(choices.iter()) {
			source.build(dir, choices, buildtype).await?
		}

		Ok(())
	}
}
