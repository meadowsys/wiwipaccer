#![allow(non_upper_case_globals)]

use crate::theme;
use serde::{ Deserialize, Serialize };
use surrealdb::engine::local::RocksDb;
use surrealdb::engine::local::Db;
use surrealdb::opt::PatchOp;
use surrealdb::sql::{ Datetime, Id, Thing, Value };
use surrealdb::Surreal;
use std::collections::BTreeMap;
use tauri::async_runtime;
use tokio::sync::RwLock;

const NS: &str = "e";
const DB: &str = "h";
const RECENTS: &str = "recents";
const DATA: &str = "data";
const THEME_SETTING: (&str, &str) = (DATA, "theme");
const THEMES: &str = "themes";

lazy_static::lazy_static! {
	static ref DATASTORE: RwLock<Option<Surreal<Db>>> = RwLock::new(None);
}

pub fn init_db(datastore_path: &str) {
	let datastore = async_runtime::block_on(async { Surreal::new::<RocksDb>(datastore_path).await })
		.expect("Couldn't create datastore");
	async_runtime::block_on(async { datastore.use_ns(NS).use_db(DB).await })
		.expect("Couldn't set NS and DB to use");
	async_runtime::block_on(async {
		*DATASTORE.write().await = Some(datastore);
	});
}

pub fn drop_db() {
	let datastore = async_runtime::block_on(DATASTORE.write()).take().unwrap();
	drop(datastore);
	eprintln!("dropped datastore");
}

pub async fn get_recent_projects() -> Vec<String> {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let mut res = datastore.query("select * from recents order by time desc")
		.await
		.unwrap();

	let recents: Vec<Recent> = res.take(0).unwrap();

	recents.into_iter()
		.map(|r| r.path)
		.collect()
}

pub async fn add_recent_project(project_path: &str) {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let recent: Option<Recent> = datastore.select((RECENTS, project_path))
		.await
		.unwrap();

	let _: Recent = if let Some(recent) = recent {
		datastore.update((RECENTS, project_path))
			.content(Recent {
				path: recent.path,
				time: Datetime::default()
			})
			.await
			.unwrap()
	} else {
		datastore.create((RECENTS, project_path))
			.content(dbg!(Recent {
				path: project_path.into(),
				time: Datetime::default()
			}))
			.await
			.unwrap()
	};
}

pub async fn remove_recent_project(project_path: &str) {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let _: Recent = datastore.delete((RECENTS, project_path))
		.await
		.unwrap();
}

pub async fn clear_recent_projects() {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let _: Vec<Recent> = datastore.delete(RECENTS)
		.await
		.unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct Recent {
	path: String,
	time: Datetime
}

pub async fn get_theme(name: String) -> theme::Theme {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let theme: Theme = datastore.select((THEMES, &name))
		.await
		.unwrap();

	convert_db_theme_to_theme(theme).await
}

pub async fn get_all_themes() -> Vec<theme::CustomTheme> {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	datastore.select(THEMES)
		.await
		.unwrap()
}

pub async fn add_or_update_theme(theme: theme::CustomTheme) {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let id = &theme.name;

	let fetched_theme: Option<theme::CustomTheme> = datastore.select((THEMES, id))
		.await
		.unwrap();

	let _: theme::CustomTheme = if let Some(theme) = fetched_theme {
		datastore.update((THEMES, id))
			.content(theme)
			.await
			.unwrap()
	} else {
		datastore.create((THEMES, id))
			.content(theme)
			.await
			.unwrap()
	};
}

pub async fn get_theme_setting() -> theme::ThemeSetting {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let setting: Option<ThemeSettingWrapper> = datastore.select(THEME_SETTING)
		.await
		.unwrap();

	let setting = if let Some(setting) = setting {
		setting
	} else {
		datastore.create(THEME_SETTING)
			.content(ThemeSettingWrapper {
				theme: ThemeSetting::System {
					light: Theme::Light,
					dark: Theme::Dark
				}
			})
			.await
			.unwrap()
	};

	convert_db_setting_to_setting(setting.theme).await
}

pub async fn set_theme_setting(setting: theme::ThemeSetting) {
	let setting = ThemeSettingWrapper {
		theme: convert_setting_to_db_setting(setting).await
	};

	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	// let _: ThemeSettingWrapper = datastore.create(THEME_SETTING)
	// 	.content(setting)
	// 	.await
	// 	.unwrap();

	let stored: Option<ThemeSettingWrapper> = datastore.select(THEME_SETTING)
		.await
		.unwrap();

	let _: ThemeSettingWrapper = if stored.is_some() {
		datastore.update(THEME_SETTING)
			.content(setting)
			.await
			.unwrap()
	} else {
		datastore.create(THEME_SETTING)
			.content(setting)
			.await
			.unwrap()
	};
}

async fn convert_theme_to_db_theme(theme: theme::Theme) -> Theme {
	match theme {
		theme::Theme::Light => { Theme::Light }
		theme::Theme::Dark => { Theme::Dark }
		theme::Theme::CustomID(id) => { Theme::Custom(Thing {
			tb: THEMES.into(),
			id: Id::String(id)
		}) }
		theme::Theme::Custom(theme) => {
			let id = theme.name.clone();
			add_or_update_theme(*theme).await;

			Theme::Custom(Thing {
				tb: THEMES.into(),
				id: Id::String(id)
			})
		}
	}
}

async fn convert_db_theme_to_theme(theme: Theme) -> theme::Theme {
	match theme {
		Theme::Light => { theme::Theme::Light }
		Theme::Dark => { theme::Theme::Dark }
		Theme::Custom(id) => {
			let datastore = DATASTORE.read()
				.await;
			let datastore = datastore.as_ref()
				.unwrap();

			let theme = datastore.select(id)
				.await
				.unwrap();

			theme::Theme::Custom(Box::new(theme))
		}
	}
}

async fn convert_setting_to_db_setting(setting: theme::ThemeSetting) -> ThemeSetting {
	match setting {
		theme::ThemeSetting::Single(theme) => {
			ThemeSetting::Single(convert_theme_to_db_theme(theme).await)
		}
		theme::ThemeSetting::System { light, dark } => {
			let light = convert_theme_to_db_theme(light).await;
			let dark = convert_theme_to_db_theme(dark).await;

			ThemeSetting::System { light, dark }
		}
	}
}

async fn convert_db_setting_to_setting(setting: ThemeSetting) -> theme::ThemeSetting {
	match setting {
		ThemeSetting::Single(theme) => { theme::ThemeSetting::Single(convert_db_theme_to_theme(theme).await) }
		ThemeSetting::System { light, dark } => { theme::ThemeSetting::System {
			light: convert_db_theme_to_theme(light).await,
			dark: convert_db_theme_to_theme(dark).await
		} }
	}
}

#[derive(Deserialize, Serialize)]
struct ThemeSettingWrapper {
	theme: ThemeSetting
}

#[derive(Deserialize, Serialize)]
enum ThemeSetting {
	Single(Theme),
	System {
		light: Theme,
		dark: Theme
	}
}

#[derive(Deserialize, Serialize)]
enum Theme {
	Light,
	Dark,
	Custom(Thing)
}
