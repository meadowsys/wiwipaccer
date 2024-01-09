use crate::error::*;
use ::std::result::Result as StdResult;
use ::tauri::{ AppHandle, Manager as _, Runtime, Window, WindowBuilder, WindowUrl };

const START_LABEL: &str = "start";
const START_URL: &str = "start";
const WORKSPACE_LABEL_PREFIX: &str = "workspace-";
const WORKSPACE_URL: &str = "workspace";

pub enum OpenOpts {
	Start,
	Workspace {
		path: String
	}
}

/// apparently on Windows™, creating a window builder deadlocks when used in a
/// synchronous command smh, so making this fn async will force an async call
/// context, even if block_on is used
pub async fn open<R: Runtime>(handle: &AppHandle<R>, opts: OpenOpts) -> Window<R> {
	use OpenOpts::*;
	let (label, url) = match opts {
		Start => {
			(START_LABEL.into(), START_URL.into())
		}
		Workspace { path } => {
			(encode_workspace_label(path), WORKSPACE_URL.into())
		}
	};

	common_builder(handle, label, url)
		.await
		.map(build_window)
		.unwrap_or_else(unminimise_and_focus)
}

#[inline]
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
		.title("");
		// ?????
		// .disable_file_drop_handler()
	Ok(builder)
}

#[inline]
pub fn encode_workspace_label(path: String) -> String {
	format!("{WORKSPACE_LABEL_PREFIX}{}", hex::encode(path))
}

#[inline]
pub fn decode_workspace_label(label: String) -> Option<String> {
	if let Some(path_hex) = label.strip_prefix(WORKSPACE_LABEL_PREFIX) {
		let path = hex::decode(path_hex).ok()?;
		let path = String::from_utf8(path).ok()?;
		Some(path)
	} else {
		None
	}
}

#[inline]
fn build_window<R: Runtime>(builder: WindowBuilder<R>) -> Window<R> {
	builder.build()
		.expect("window failed to build")
}

fn unminimise_and_focus<R: Runtime>(window: Window<R>) -> Window<R> {
	window.unminimize().expect("couldn't unminimise window");
	window.set_focus().expect("couldn't focus window");
	window
}
