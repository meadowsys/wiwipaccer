use crate::error::*;
use super::AppDB;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct SavedWorkspace {
	config: String
}

const WORKSPACE_TABLE: &str = "workspaces";

impl SavedWorkspace {
	#[inline]
	pub fn new(config: String) -> Self {
		Self { config }
	}

	pub async fn read(db: &AppDB, name: &str) -> Result<Option<Self>> {
		let surreal = db.surreal().await;
		let config: Option<Self> = surreal.select((WORKSPACE_TABLE, name)).await?;

		Ok(config)
	}

	pub async fn write(&self, name: &str, db: &AppDB) -> Result<()> {
		let record_id = (WORKSPACE_TABLE, name);

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

	pub async fn list(db: &AppDB) -> Result<Vec<String>> {
		#[derive(Deserialize)]
		struct IDOnly {
			id: String
		}

		let surreal = db.surreal().await;
		let mut res = surreal.query("select meta::id(id) from $table")
			.bind(("table", WORKSPACE_TABLE))
			.await?;
		let ids: Vec<IDOnly> = res.take(0)?;

		Ok(ids.into_iter().map(|id| id.id).collect())
	}

	#[inline]
	pub fn into_inner(self) -> String {
		self.config
	}
}
