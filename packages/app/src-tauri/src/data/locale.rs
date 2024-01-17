use crate::error::*;
use super::AppDB;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct LocaleSetting {
	locales: DBType
}
type DBType = Vec<String>;

const LOCALE_RECORD_ID: (&str, &str) = (super::SETTINGS_TABLE, "app-locale");

impl LocaleSetting {
	#[inline]
	pub fn new(locales: Vec<String>) -> Self {
		Self { locales }
	}

	pub async fn read_or_default(db: &AppDB) -> Result<Self> {
		let surreal = db.surreal().await;
		let locales: Option<DBType> = surreal.select(LOCALE_RECORD_ID).await?;

		let locales = if let Some(locales) = locales {
			locales
		} else {
			let locales: Option<DBType> = surreal.create(LOCALE_RECORD_ID)
				.content(Self::default())
				.await?;
			locales.unwrap()
		};

		Ok(Self { locales })
	}

	pub async fn write(&self, db: &AppDB) -> Result<()> {
		let surreal = db.surreal().await;
		let _: Option<DBType> = surreal.update(LOCALE_RECORD_ID)
			.content(self)
			.await?;
		Ok(())
	}

	#[inline]
	pub fn into_inner(self) -> Vec<String> {
		self.locales
	}
}

impl Default for LocaleSetting {
	#[inline]
	fn default() -> Self {
		let locales = vec!["en".into()];
		Self { locales }
	}
}
