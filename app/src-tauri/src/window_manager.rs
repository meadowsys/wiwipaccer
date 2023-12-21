use tauri::{ AppHandle, Runtime, WindowBuilder, WindowUrl };

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
pub fn open_welcome_window_temporary<R: Runtime>(app: &AppHandle<R>) {
	let builder = WindowBuilder::new(app, "welcome-temp", WindowUrl::App("".into()))
		.accept_first_mouse(false)
		.enable_clipboard_access()
		.title("");

	#[cfg(target_os = "macos")]
	let builder = builder.title_bar_style(TitleBarStyle::Overlay);

	builder
		.inner_size(800., 500.)
		.build()
		.unwrap();
}
