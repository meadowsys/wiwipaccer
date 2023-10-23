use crate::*;
use tauri::{ AppHandle, Runtime, Window, WindowBuilder, WindowEvent, WindowUrl, async_runtime, Manager };

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};


const WELCOME_WINDOW_LABEL: &str = "welcome_window";
const DOCS_WINDOW_LABEL: &str = "docs";
const SETTINGS_WINDOW_LABEL: &str = "settings";

pub fn open_welcome_window<R: Runtime>(app: &AppHandle<R>) {
	let builder = get_window_builder(app, WELCOME_WINDOW_LABEL, WindowUrl::App("welcome".into()))
		.inner_size(850., 500.)
		.min_inner_size(850., 500.);

	#[cfg(target_os = "macos")]
	let builder = builder.title_bar_style(TitleBarStyle::Overlay);

	let window = build_and_etc(app, builder);

	{
		let app = app.clone();
		window.on_window_event(move |event| {
			match event {
				WindowEvent::Focused(focused) if *focused => {
					// app.set_menu(menu::welcome_menu_bar(&app))
					// 	.expect("unable to set app menu");
				}
				_ => {}
			}
		});
	}

	async_runtime::spawn(theme::set_system_theme(app.clone(), window.theme().unwrap()));
}

pub fn get_welcome_window<R: Runtime>(app: &AppHandle<R>) -> Option<Window<R>> {
	app.get_window(WELCOME_WINDOW_LABEL)
}

pub fn open_about_window<R: Runtime>(app: &AppHandle<R>) {
	lazy_static::lazy_static! {
		static ref ABOUT_WINDOW_LABEL: String = format!("about-{}", hex::encode(super::ACTUAL_APP_VERSION));
	}

	if let Some(window) = app.get_window(&ABOUT_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus the window");
	} else {
		let builder = get_window_builder(app, &ABOUT_WINDOW_LABEL, WindowUrl::App("about".into()))
			// .transparent(false)
			.inner_size(550., 350.)
			.resizable(false);
			// .min_inner_size(750., 350.)
		let _window = build_and_etc(app, builder);
	}
}

pub fn open_docs_window<R: Runtime>(app: &AppHandle<R>) {
	if let Some(window) = app.get_window(DOCS_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus docs window");
	} else {
		let builder = get_window_builder(app, DOCS_WINDOW_LABEL, WindowUrl::App("docs".into()))
			// .transparent(false)
			.inner_size(800., 500.)
			.min_inner_size(800., 500.);
		let _window = build_and_etc(app, builder);
	}
}

pub fn open_project_window<R: Runtime>(app: &AppHandle<R>, path: &str) {
	let label = hex::encode(path);
	let existing = app.get_window(&label);
	if let Some(window) = existing {
		window.set_focus()
			.expect("couldn't focus project window");
	} else {
		// TODO maybe send a signal back to main or something if this is Err
		// so that user gets an alert that opening it failed
		let builder = get_window_builder(app, &label, WindowUrl::App("project-folder".into()))
			// .transparent(false)
			.inner_size(800., 500.)
			.min_inner_size(800., 500.);
		let _window = build_and_etc(app, builder);
	}
}

pub fn open_settings_window<R: Runtime>(app: &AppHandle<R>) {
	if let Some(window) = app.get_window(SETTINGS_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus settings window");
	} else {
		let builder = get_window_builder(app, SETTINGS_WINDOW_LABEL, WindowUrl::App("settings".into()))
			.inner_size(1000., 600.)
			.min_inner_size(700., 500.);
		let _window = build_and_etc(app, builder);
	}
}

fn get_window_builder<'h, R: Runtime>(app: &'h AppHandle<R>, label: &'h str, url: WindowUrl) -> WindowBuilder<'h, R> {
	let builder = WindowBuilder::new(app, label, url)
		.accept_first_mouse(false)
		.enable_clipboard_access()
		// .transparent(true)
		.title("");

	#[cfg(target_os = "macos")]
	let builder = builder.title_bar_style(TitleBarStyle::Overlay);

	builder
}

fn build_and_etc<R: Runtime>(app: &AppHandle<R>, builder: WindowBuilder<'_, R>) -> Window<R> {
	let window = builder.build().unwrap();

	let app = app.clone();
	#[allow(clippy::single_match)]
	window.on_window_event(move |event| match event {
		WindowEvent::ThemeChanged(theme) => {
			async_runtime::spawn(theme::emit_update_theme(app.clone(), *theme));
		}
		_ => {}
	});
	// window.on_menu_event(f)

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
