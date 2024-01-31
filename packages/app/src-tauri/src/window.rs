use ::std::result::Result as StdResult;
use ::tauri::{ AppHandle, Manager as _, Runtime, Window, WindowBuilder, WindowUrl };

// #[cfg(target_os = "macos")]
// use ::tauri::TitleBarStyle;

pub enum WindowType {
	Start,
	Workspace {
		name: String
	}
}

/// apparently on Windows™, creating a window builder deadlocks when used in a
/// synchronous command smh, so making this fn async will force an async call
/// context, even if block_on is used
pub async fn open<R: Runtime>(handle: &AppHandle<R>, window_type: WindowType) -> Window<R> {
	let (label, url) = window_type.get_label_and_url();

	common_builder(handle, label, url)
		.await
		.map(build_window)
		.unwrap_or_else(unminimise_and_focus)
}

async fn common_builder<R: Runtime>(
	handle: &AppHandle<R>,
	label: String,
	url: String
) -> StdResult<WindowBuilder<R>, Window<R>> {
	if let Some(window) = handle.get_window(&label) {
		return Err(window)
	}

	let builder = WindowBuilder::new(handle, label, WindowUrl::App(url.into()))
		.accept_first_mouse(false)
		.enable_clipboard_access()
		.hidden_title(true)
		// .decorations(false)
		// .transparent(true)
		.min_inner_size(800., 500.)
		.title("")
		.disable_file_drop_handler();

	// #[cfg(target_os = "macos")]
	// let builder = builder.title_bar_style(TitleBarStyle::Overlay);

	Ok(builder)
}

#[inline]
fn build_window<R: Runtime>(builder: WindowBuilder<R>) -> Window<R> {
	builder.build()
		.expect("window failed to build")
}

#[inline]
fn unminimise_and_focus<R: Runtime>(window: Window<R>) -> Window<R> {
	window.unminimize().expect("couldn't unminimise window");
	window.set_focus().expect("couldn't focus window");
	window
}

impl WindowType {
	const START_LABEL: &'static str = "start";
	const START_URL: &'static str = "start";
	const WORKSPACE_LABEL_PREFIX: &'static str = "workspace-";
	const WORKSPACE_URL: &'static str = "workspace";

	pub fn get_label_and_url(&self) -> (String, String) {
		use WindowType::*;

		match self {
			Start => { (
				Self::START_LABEL.into(),
				Self::START_URL.into()
			) }
			Workspace { name } => { (
				format!(
					"{prefix}{name}",
					prefix = Self::WORKSPACE_LABEL_PREFIX
				),
				Self::WORKSPACE_URL.into()
			) }
		}
	}

	pub fn decode_window_label(label: &str) -> Option<Self> {
		if label == Self::START_LABEL {
			Some(Self::Start)
		} else if let Some(name) = label.strip_prefix(Self::WORKSPACE_LABEL_PREFIX) {
			let name = name.into();
			Some(Self::Workspace { name })
		} else {
			None
		}
	}
}
