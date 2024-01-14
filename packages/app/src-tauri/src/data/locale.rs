use crate::error::*;
use super::AppDB;
use ::serde::{ Deserialize, Serialize };

#[derive(Clone, Deserialize, Serialize)]
pub struct LocaleSetting {
	locales: Vec<String>
}

const LOCALE_RECORD_ID: (&str, &str) = (super::SETTINGS_TABLE, "app-locale");

impl LocaleSetting {
	#[inline]
	pub fn new(locales: Vec<String>) -> Self {
		Self { locales }
	}

	pub async fn read_or_default(db: &AppDB) -> Result<Self> {
		let surreal = db.surreal().await;
		let locale: Option<Self> = surreal.select(LOCALE_RECORD_ID).await?;

		if let Some(locale) = locale {
			Ok(locale)
		} else {
			let locale: Option<Self> = surreal.create(LOCALE_RECORD_ID)
				.content(Self::default())
				.await?;
			Ok(locale.unwrap())
		}
	}

	pub async fn write(self, db: &AppDB) -> Result<Self> {
		let surreal = db.surreal().await;
		let locale: Option<Self> = surreal.update(LOCALE_RECORD_ID)
			.content(self)
			.await?;
		Ok(locale.unwrap())
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
