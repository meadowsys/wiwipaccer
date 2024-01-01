// use async_trait::async_trait;
// use camino::Utf8PathBuf;
// use crate::error::{ self, Error, Result };
// use crate::pack_sources::{ self, Source };
// use crate::ron;
// use crate::util;
// use hashbrown::HashMap;
// use semver::VersionReq;
// use serde::{ Deserialize, Serialize };

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "meta_version")]
// pub enum WorkspaceConfig {
// 	#[serde(rename = "1")]
// 	Version1 {
// 		name: String,
// 		projects: Vec<String>
// 	}
// }

// pub struct Workspace {
// 	name: String,
// 	sources: HashMap<String, Source>,
// 	source_ids: Vec<String>
// }

// impl Workspace {
// 	pub fn new(name: String) -> Workspace {
// 		let sources = HashMap::new();
// 		let source_ids = Vec::new();

// 		Workspace { name, sources, source_ids }
// 	}

// 	pub async fn from_config(config: WorkspaceConfig) -> Result<Self> {
// 		let new = match config {
// 			WorkspaceConfig::Version1 { name, projects } => {
// 				let mut new = Self::new(name);

// 				for project in projects {
// 					new.add_source(project).await?;
// 				}

// 				new
// 			}
// 		};

// 		Ok(new)
// 	}

// 	pub fn to_config(&self) -> WorkspaceConfig {
// 		let name = self.name.clone();
// 		let projects = self.source_ids.iter()
// 			.map(|path| self.sources.get(path).expect("invalid state"))
// 			.map(|source| source.pack_id().into())
// 			.collect::<Vec<_>>();
// 		WorkspaceConfig::Version1 { name, projects }
// 	}

// 	pub async fn add_source(&mut self, dir: String) -> Result<()> {
// 		let sources = &self.sources;
// 		let resolver = DependencyResolver { sources };

// 		let source = Source::new(dir, resolver).await?;
// 		let name = source.pack_id().to_owned();

// 		self.sources.insert(name.clone(), source);
// 		self.source_ids.push(name);

// 		Ok(())
// 	}
// }

// struct DependencyResolver<'h> {
// 	sources: &'h HashMap<String, Source>
// }

// struct Dependency<'h> {
// 	source: &'h Source
// }

// #[async_trait]
// impl<'h> pack_sources::DependencyResolver for DependencyResolver<'h> {
// 	type Dependency = Dependency<'h>;
// 	async fn depedency(&self, pack_id: &str, req: &VersionReq) -> Result<Option<Self::Dependency>> {
// 		let source = match self.sources.get(pack_id) {
// 			Some(s) => { s }
// 			None => { return Ok(None) }
// 		};

// 		if !req.matches(source.version()) { return Ok(None) }

// 		let dependency = Dependency { source };
// 		Ok(Some(dependency))
// 	}
// }

// #[async_trait]
// impl<'h> pack_sources::Dependency for Dependency<'h> {}
