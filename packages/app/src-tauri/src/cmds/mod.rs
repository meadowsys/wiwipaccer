pub mod locale;
pub mod window;
pub mod workspace;

#[macro_export]
macro_rules! command_handler {
	() => {{
		use $crate::cmds;

		::tauri::generate_handler![
			cmds::locale::read_locale_setting,
			cmds::locale::write_locale_setting,

			cmds::workspace::list_existing_workspaces,
			cmds::workspace::check_workspace_name_is_available,
			cmds::workspace::create_new_workspace,
			cmds::workspace::open_workspace,
			cmds::workspace::get_frontend_data_for,

			cmds::window::get_workspace_name
		]
	}}
}
pub use command_handler;
