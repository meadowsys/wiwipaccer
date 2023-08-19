#![allow(unused_imports)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use mimalloc::MiMalloc;
use tauri::{ WindowBuilder, WindowUrl };
use tauri::async_runtime;
use std::fs;

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};

mod cmds;
mod db;
mod menu;
mod theme;
mod window_manager;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const APPDATA_ROOTDIR: &str = ".wiwipaccer";
const DATASTORE_PATH: &str = "data";

// WINDOWS WIX DOESN'T SUPPORT PROPER SEMVER AAAAAAAAAAAAAAAAAAAA
const ACTUAL_APP_VERSION: &str = include_str!("../meta/version");

const UPDATER_ENABLED: bool = include!("../meta/updater-enabled");
const UPDATER_NICE_PLATFORM_NAME: &str = include_str!("../meta/updater-nice-platform-name");

// static DATASTORE: Surreal<RocksDb> = Surreal::new();

fn main() {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.build()
		.unwrap();
	async_runtime::set(rt.handle().clone());

	#[cfg(not(debug_assertions))]
	let mut appdata_rootdir = tauri::api::path::home_dir()
		.expect("cannot get home directory")
		.to_str()
		.expect("path is not valid UTF-8, only valid UTF-8 pathnames are supported")
		.to_string();
	#[cfg(debug_assertions)]
	let mut appdata_rootdir = {
		let mut appdata_rootdir = std::env::current_dir()
			.expect("couldn't get current dir")
			.to_str()
			.expect("path is not valid UTF-8, only valid UTF-8 pathnames are supported")
			.to_string();

		appdata_rootdir.push_str("/dev-datadir");

		if fs::metadata(&appdata_rootdir).is_err() {
			fs::create_dir(&appdata_rootdir)
				.unwrap_or_else(|_| panic!("couldn't create appdata_rootdir for dev: {appdata_rootdir}"));
		}

		appdata_rootdir
	};

	appdata_rootdir.reserve(APPDATA_ROOTDIR.len() + DATASTORE_PATH.len() + 2);
	appdata_rootdir.push('/');
	appdata_rootdir.push_str(APPDATA_ROOTDIR);

	match fs::metadata(&appdata_rootdir) {
		Ok(meta) => match meta.is_dir() {
			true => { /* assuming its ours */ }
			false => { panic!("path is already taken, isn't dir: {appdata_rootdir}") }
		}
		Err(_) => {
			// probably doesn't exist?
			fs::create_dir(&appdata_rootdir)
				.unwrap_or_else(|_| panic!("couldn't create root appdata dir {appdata_rootdir}"));
		}
	}

	let mut datastore_path = appdata_rootdir;
	datastore_path.push('/');
	datastore_path.push_str(DATASTORE_PATH);

	db::init_db(&datastore_path);

	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_window::init())
		.setup(|app| {
			let app = app.handle();
			window_manager::open_welcome_window(app);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			cmds::add_recent_project,
			cmds::clear_recent_projects,
			cmds::decode_hex_string,
			cmds::get_license,
			cmds::get_platform,
			cmds::get_project_meta,
			cmds::get_project_basic_meta,
			cmds::get_recent_projects,
			cmds::get_theme,
			cmds::open_about,
			cmds::open_docs,
			cmds::open_project,
			cmds::remove_recent_project
		])
		.build(tauri::generate_context!())
		.expect("error while running application")
		.run(|_app, event| {
			use tauri::RunEvent;

			#[allow(clippy::single_match)]
			match event {
				RunEvent::Exit => {
					db::drop_db();
				}
				_ => {}
			}
		});
}
