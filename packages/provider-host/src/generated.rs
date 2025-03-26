#![allow(
	unused_imports,
	reason = "prelude of sorts (todo remove me?)"
)]

mod __generated {
	wasmtime::component::bindgen!({
		path: "../provider-api/provider-plugin.wit",

		// todo toy with configs if necessary
		ownership: Borrowing {
			duplicate_if_necessary: true
		}
	});
}

pub use self::__generated::{ Plugin, PluginIndices, PluginPre };
pub use self::__generated::exports::wiwi::wiwipaccer_provider_plugin::imp::{ Guest, GuestIndices };
pub use self::__generated::wiwi::wiwipaccer_provider_plugin::api::{
	Host as HostApi,
	add_to_linker as add_to_linker_api
};
pub use self::__generated::wiwi::wiwipaccer_provider_plugin::types::{
	Asset,
	Error,
	HostAsset,
	HostError,
	HostManifestMap,
	HostRunCtx,
	ManifestKey,
	ManifestMap,
	ManifestSubItem,
	PackEntries,
	PluginMeta,
	RunCtx,
	Host as HostTypes,
	add_to_linker as add_to_linker_types
};
