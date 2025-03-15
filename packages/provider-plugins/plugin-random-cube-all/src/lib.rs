use wiwipaccer_provider_plugin_impl::{ Guest, PluginMetadata, export };

struct Plugin;
export!(Plugin);

impl Guest for Plugin {
	fn metadata() -> PluginMetadata {
		PluginMetadata {
			ns: "wiwi".into(),
			id: "random-cube-all".into(),
			version: "0.1.0-testlol".into()
			// authors: vec![
			// 	"Meadow Liu <meadowsys@kiwin.gay>".into(),
			// 	api::h()
			// ]
		}
	}
}
