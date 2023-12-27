use camino::Utf8PathBuf;
use crate::error::{ self, Error, Result };
use crate::pack_sources::Source;
use crate::ron;
use crate::util;
use hashbrown::HashMap;
use semver::VersionReq;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub enum WorkspaceConfig {
	#[serde(rename = "1")]
	Version1 {
		projects: Vec<ProjectConfigEntry>
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfigEntry {
	pub name: Option<String>,
	pub path: String
}

pub struct Workspace {
	sources: HashMap<String, Source>
}

impl Workspace {
	pub fn new() -> Workspace {
		let sources = HashMap::new();

		Workspace { sources }
	}

	pub async fn from_config(config: WorkspaceConfig) -> Result<Self> {
		let new = Self::new();
		todo!()
	}

	pub fn to_config(&self) -> WorkspaceConfig {
		// WorkspaceConfig::Version1 {}
		todo!()
	}

	pub async fn add_source(&mut self, dir: Utf8PathBuf) -> Result<()> {
		let source = Source::new(dir).await?;
		let name = source.name().into();

		self.sources.insert(name, source);

		Ok(())
	}
}
