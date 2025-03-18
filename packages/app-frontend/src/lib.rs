#![feature(panic_update_hook)]

use leptos::prelude::*;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn main() {
	panic::update_hook(|prev_hook, info| {
		console_error_panic_hook::hook(info);
		prev_hook(info);
	});

	mount_to_body(|| view! {
		<div data-tauri-drag-region>
			{ "uwuwuwuwuwu" }
		</div>
	});
}
