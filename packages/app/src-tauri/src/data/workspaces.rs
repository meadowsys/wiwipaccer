use crate::error::*;
use super::AppDB;
use ::serde::{ Deserialize, Serialize };
use ::wiwipaccer_core::workspace::WorkspaceConfig;

#[derive(Deserialize, Serialize)]
pub struct SavedWorkspace {
	config: WorkspaceConfig
}

const WORKSPACE_TABLE: &str = "workspaces";

impl SavedWorkspace {
	#[inline]
	pub fn new(config: WorkspaceConfig) -> Self {
		Self { config }
	}

	pub async fn read_and_load(db: &AppDB, name: &str) -> Result<Option<Self>> {
		let surreal = db.surreal().await;
		let config: Option<Self> = surreal.select((WORKSPACE_TABLE, name)).await?;

		Ok(config)
	}

	pub async fn write(&self, db: &AppDB) -> Result<()> {
		let record_id = (WORKSPACE_TABLE, self.config.name().ref_inner());

		let surreal = db.surreal().await;
		let config: Option<Self> = surreal.select(record_id).await?;

		let _: Option<Self> = if config.is_some() {
			surreal.update(record_id)
				.content(self)
				.await?
		} else {
			surreal.create(record_id)
				.content(self)
				.await?
		};

		Ok(())
	}

	#[inline]
	pub fn into_inner(self) -> WorkspaceConfig {
		self.config
	}
}
