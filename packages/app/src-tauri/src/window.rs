use tauri::{ AppHandle, Runtime, Window, WindowBuilder, WindowUrl };
use crate::error::*;

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
pub async fn open<R: Runtime>(handle: &AppHandle<R>, opts: OpenOpts) -> Result<Window<R>> {
	use OpenOpts::*;
	match opts {
		Start => {
			let window = common_builder(handle, START_LABEL.into(), START_URL.into())
				.await
				.build_window();
			Ok(window)
		}
		Workspace { path } => {
			let label = encode_workspace_label(path);
			let window = common_builder(handle, label, WORKSPACE_URL.into())
				.await
				.build_window();
			Ok(window)
		}
	}
}

#[inline]
async fn common_builder<R: Runtime>(
	handle: &AppHandle<R>,
	label: String,
	url: String
) -> WindowBuilder<R> {
	WindowBuilder::new(handle, label, WindowUrl::App(url.into()))
		.accept_first_mouse(false)
		.enable_clipboard_access()
		.title("")
		// ?????
		// .disable_file_drop_handler()
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

trait WindowBuilderExt<R: Runtime> {
	fn build_window(self) -> Window<R>;
}

impl<R: Runtime> WindowBuilderExt<R> for WindowBuilder<'_, R> {
	#[inline]
	fn build_window(self) -> Window<R> {
		self.build()
			.expect("window failed to build")
	}
}
