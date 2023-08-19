use tauri::{ AppHandle, Runtime, Window, WindowBuilder, WindowEvent, WindowUrl, async_runtime };

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};

pub fn get_window_builder<'h, R: Runtime>(app: &'h AppHandle<R>, label: &'h str, url: WindowUrl) -> WindowBuilder<'h, R> {
	let builder = WindowBuilder::new(app, label, url)
		.accept_first_mouse(false)
		.enable_clipboard_access()
		// .transparent(true)
		.title("");

	#[cfg(target_os = "macos")]
	let builder = builder.title_bar_style(TitleBarStyle::Overlay);

	builder
}

pub fn build_and_etc<R: Runtime>(apphandle: AppHandle<R>, builder: WindowBuilder<'_, R>) -> Window<R> {
	let window = builder.build().unwrap();

	#[allow(clippy::single_match)]
	window.on_window_event(move |event| match event {
		WindowEvent::ThemeChanged(theme) => {
			async_runtime::spawn(crate::theme::emit_update_theme(apphandle.clone(), *theme));
		}
		_ => {}
	});

	window
}

// #[allow(unused)]
// pub fn apply_relevant_window_effects<R: Runtime>(app: &AppHandle<R>, window: Window<R>) {
// 	#[cfg(target_os = "macos")]
// 	app.run_on_main_thread(move || {
// 		apply_vibrancy(
// 			&window,
// 			NSVisualEffectMaterial::HudWindow,
// 			None,
// 			None
// 		).expect("apply_vibrancy is mac only lol");
// 	}).unwrap();
// }
