use wiwipaccer_provider_plugin_api::{ Guest, PluginMetadata, cargo_pkg_version, export };

struct Plugin;
export!(Plugin);

impl Guest for Plugin {
	fn metadata() -> PluginMetadata {
		PluginMetadata {
			ns: "wiwi".into(),
			id: "random-cube-all".into(),
			version: cargo_pkg_version!()
			// authors: vec![
			// 	"Meadow Liu <meadowsys@kiwin.gay>".into(),
			// 	api::h()
			// ]
		}
	}
}
