#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use mimalloc::MiMalloc;
use surrealdb::Datastore;
use tauri::{ TitleBarStyle, WindowBuilder, WindowUrl };
use tauri::async_runtime;
use std::fs;
use tokio::sync::RwLock;
use window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial };

mod cmds;
mod db;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const APPDATA_ROOTDIR: &str = ".wiwipaccer";
const DATASTORE_PATH: &str = "data";
const WELCOME_WINDOW_NAME: &str = "welcome_window";

lazy_static::lazy_static! {
	static ref DATASTORE: RwLock<Option<Datastore>> = RwLock::new(None);
}

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

	appdata_rootdir.reserve(appdata_rootdir.len() + DATASTORE_PATH.len() + 2);
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
				.unwrap_or_else(|_| panic!("couldn't create root appdata dir ~/{APPDATA_ROOTDIR}"));
		}
	}

	let mut datastore_path = appdata_rootdir;
	datastore_path.push('/');
	datastore_path.push_str(DATASTORE_PATH);

	let datastore = async_runtime::block_on(Datastore::new(&format!("file://{datastore_path}")))
		.expect("Couldn't create datastore");
	async_runtime::block_on(async {
		*DATASTORE.write().await = Some(datastore);
	});

	tauri::Builder::default()
		.setup(|app| {
			let welcome_window = WindowBuilder::new(app, WELCOME_WINDOW_NAME, WindowUrl::App("welcome".into()))
				.accept_first_mouse(false)
				.enable_clipboard_access()
				.title_bar_style(TitleBarStyle::Overlay)
				.inner_size(800., 500.)
				.min_inner_size(800., 500.)
				.title("")
				.transparent(true)
				.build()?;

			#[cfg(target_os = "macos")]
			apply_vibrancy(
				&welcome_window,
				NSVisualEffectMaterial::HudWindow,
				None,
				None
			).expect("apply_vibrancy is mac only lol");

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			cmds::add_recent_project,
			cmds::decode_hex_string,
			cmds::get_recent_projects,
			cmds::open_project
		])
		.build(tauri::generate_context!())
		.expect("error while running application")
		.run(|_apphandle, event| {
			use tauri::RunEvent;

			#[allow(clippy::single_match)]
			match event {
				RunEvent::Exit => {
					let datastore = async_runtime::block_on(DATASTORE.write()).take().unwrap();
					drop(datastore);
					eprintln!("dropped datastore");
				}
				_ => {}
			}
		});
}

// let system_tray_menu = {
// 	let e = tauri::CustomMenuItem::new("e", "e");
// 	tauri::SystemTrayMenu::new()
// 		.add_item(e)
// };

// tauri::SystemTray::new()
// 	.with_menu(system_tray_menu)
// 	.build(apphandle)
// 	.unwrap();
