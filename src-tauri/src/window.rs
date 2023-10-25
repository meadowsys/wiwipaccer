use tauri::{ AppHandle, Runtime, TitleBarStyle, Window, WindowBuilder, WindowUrl, Manager };

#[inline]
pub fn try_welcome<R: Runtime>(app: &AppHandle<R>) -> Option<Window<R>> {
	app.get_window(consts::WELCOME_LABEL)
}

#[inline]
pub fn welcome<R: Runtime>(app: &AppHandle<R>) -> Window<R> {
	try_welcome(app)
		.unwrap_or_else(|| {
			let builder = WindowBuilder::new(app, consts::WELCOME_LABEL, WindowUrl::App(consts::WELCOME_URL.into()));
			let builder = standard_transformations(builder);
			standard_build(builder)
		})
}

pub mod consts {
	pub const WELCOME_LABEL: &str = "welcome";
	pub const WELCOME_URL: &str = "welcome";
}

#[inline]
fn standard_transformations<R: Runtime>(builder: WindowBuilder<R>) -> WindowBuilder<R> {
	builder
		.accept_first_mouse(false)
		.enable_clipboard_access()
		.title("")
		.title_bar_style(TitleBarStyle::Overlay)
}

#[inline]
fn standard_build<R: Runtime>(builder: WindowBuilder<R>) -> Window<R> {
	builder.build().unwrap()
}
