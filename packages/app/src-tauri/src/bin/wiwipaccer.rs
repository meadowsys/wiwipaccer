#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use ::tauri::{ async_runtime, Manager, RunEvent };
use ::wiwipaccer::*;
use ::wiwipaccer::window::OpenOpts;

#[cfg(not(debug_assertions))]
#[global_allocator]
static ALLOC: ::mimalloc::MiMalloc = ::mimalloc::MiMalloc;

fn main() {
	let rt = rt::create_rt();
	async_runtime::set(rt.handle().clone());

	tauri::Builder::default()
		.setup(|app| {
			let handle = app.handle();

			let appdb = async_runtime::block_on(data::AppDB::new(handle))
				.expect("failed to create db");
			let workspaces = workspaces::Workspaces::new(&appdb);

			app.manage(appdb);
			app.manage(workspaces);

			let window_future = window::open(handle, OpenOpts::Start);
			let _window = async_runtime::block_on(window_future);

			Ok(())
		})
		.invoke_handler(cmds::command_handler!())
		.build(tauri::generate_context!())
		.expect("error running app")
		.run(|app, event| {
			// there will be more handlers implemented later
			// TODO: remove this clippy lint lol
			#[allow(clippy::single_match)]
			match event {
				RunEvent::Exit => {
					app.state::<data::AppDB>()
						.drop_db();
				}
				_ => {}
			}
		})
}
