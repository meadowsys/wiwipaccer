use serde::{ Deserialize, Serialize };
use tauri::Manager;
use tauri::AppHandle;
use tauri::async_runtime;
use tauri::Runtime;
use tokio::sync::Mutex;

const THEME_UPDATE: &str = "theme_update";

lazy_static::lazy_static! {
	static ref SYSTEM_THEME: Mutex<tauri::Theme> = Mutex::new(tauri::Theme::Light);
}

pub async fn get_system_theme() -> tauri::Theme {
	*SYSTEM_THEME.lock().await
}

pub async fn set_system_theme<R: Runtime>(apphandle: AppHandle<R>, theme: tauri::Theme) {
	*SYSTEM_THEME.lock().await = theme;
}

/// hacky but, if theme is Err then we skip checking for matching (aka this is init calling this)
pub async fn emit_update_theme<R: Runtime>(apphandle: AppHandle<R>, theme: tauri::Theme) {
	let mut current_theme_mutex = SYSTEM_THEME.lock().await;

	if theme == *current_theme_mutex { return }
	*current_theme_mutex = theme;

	let setting = crate::db::get_theme_setting().await;
	let theme = get_theme_from_setting(setting, theme);

	println!("emitting");
	apphandle.emit_all(THEME_UPDATE, theme).unwrap();
}

pub fn get_theme_from_setting(setting: ThemeSetting, theme: tauri::Theme) -> Theme {
	match setting {
		ThemeSetting::Single(theme) => { theme }
		ThemeSetting::System { light, dark } => match theme {
			tauri::Theme::Light => { light }
			tauri::Theme::Dark => { dark }
			_ => { unimplemented!("lol? what is this?") }
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ThemeSetting {
	Single(Theme),
	System {
		light: Theme,
		dark: Theme
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Theme {
	#[serde(rename = "light")]
	Light,
	#[serde(rename = "dark")]
	Dark,
	#[serde(rename = "custom_id")]
	CustomID(String),
	#[serde(rename = "custom")]
	Custom(Box<CustomTheme>)
}

// TODO write a macro for this?
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomTheme {
	pub name: String,
	#[serde(rename = "--p")]
	pub _p: String,
	#[serde(rename = "--pf")]
	pub _pf: String,
	#[serde(rename = "--pc")]
	pub _pc: String,
	#[serde(rename = "--s")]
	pub _s: String,
	#[serde(rename = "--sf")]
	pub _sf: String,
	#[serde(rename = "--sc")]
	pub _sc: String,
	#[serde(rename = "--a")]
	pub _a: String,
	#[serde(rename = "--af")]
	pub _af: String,
	#[serde(rename = "--ac")]
	pub _ac: String,
	#[serde(rename = "--n")]
	pub _n: String,
	#[serde(rename = "--nf")]
	pub _nf: String,
	#[serde(rename = "--nc")]
	pub _nc: String,
	#[serde(rename = "--b1")]
	pub _b1: String,
	#[serde(rename = "--b2")]
	pub _b2: String,
	#[serde(rename = "--b3")]
	pub _b3: String,
	#[serde(rename = "--bc")]
	pub _bc: String,
	#[serde(rename = "--in")]
	pub _in: String,
	#[serde(rename = "--inc")]
	pub _inc: String,
	#[serde(rename = "--su")]
	pub _su: String,
	#[serde(rename = "--suc")]
	pub _suc: String,
	#[serde(rename = "--wa")]
	pub _wa: String,
	#[serde(rename = "--wac")]
	pub _wac: String,
	#[serde(rename = "--er")]
	pub _er: String,
	#[serde(rename = "--erc")]
	pub _erc: String
}
