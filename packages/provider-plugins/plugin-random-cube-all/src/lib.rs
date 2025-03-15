use wiwipaccer_provider_plugin_impl::*;

// use exports::wiwi::wiwipaccer_provider_plugin::imp;
// use wiwi::wiwipaccer_provider_plugin::api;

struct Plugin;
export!(Plugin);

impl Guest for Plugin {
	fn metadata() -> PluginMetadata {
		PluginMetadata {
			ns: "wiwi".into(),
			id: "random-cube-all".into(),
			version: "0.1.0-testlol".into(),
			// authors: vec![
			// 	"Meadow Liu <meadowsys@kiwin.gay>".into(),
			// 	api::h()
			// ]
		}
	}
}

pub const fn å() -> &'static str {
	"aaaaaaaaaaaaaaaaaa"
}
