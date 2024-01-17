use crate::error::*;
use super::AppDB;
use ::serde::{ Deserialize, Serialize };
use ::wiwipaccer_core::workspace::WorkspaceConfig;

#[derive(Deserialize, Serialize)]
pub struct SavedWorkspace {
	config: DBType
}
type DBType = WorkspaceConfig;

const WORKSPACE_TABLE: &str = "workspaces";

impl SavedWorkspace {
	#[inline]
	pub fn new(config: WorkspaceConfig) -> Self {
		Self { config }
	}

	pub async fn read_and_load(db: &AppDB, name: &str) -> Result<Option<Self>> {
		let surreal = db.surreal().await;
		let config: Option<DBType> = surreal.select((WORKSPACE_TABLE, name)).await?;

		Ok(config.map(|config| {
			debug_assert_eq!(name, config.name().ref_inner());

			Self { config }
		}))
	}

	pub async fn write(&self, db: &AppDB) -> Result<()> {
		let record_id = (WORKSPACE_TABLE, self.config.name().ref_inner());

		let surreal = db.surreal().await;
		let config: Option<DBType> = surreal.select(record_id).await?;

		let _: Option<DBType> = if config.is_some() {
			surreal.update(record_id)
				.content(&self.config)
				.await?
		} else {
			surreal.create(record_id)
				.content(&self.config)
				.await?
		};

		Ok(())
	}

	#[inline]
	pub fn into_inner(self) -> WorkspaceConfig {
		self.config
	}
}
