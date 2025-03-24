use wiwipaccer_provider_plugin_api::{ Guest, PluginMeta, cargo_pkg_version, export };

struct Plugin;
export!(Plugin);

impl Guest for Plugin {
	fn meta() -> PluginMeta {
		PluginMeta {
			ns: "wiwi".into(),
			id: "cube-all".into(),
			version: Some(cargo_pkg_version!())
			// authors: vec![
			// 	"Meadow Liu <meadowsys@kiwin.gay>".into(),
			// 	api::h()
			// ]
		}
	}
}
