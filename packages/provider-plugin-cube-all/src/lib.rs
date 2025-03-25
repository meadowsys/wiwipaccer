use wiwipaccer_provider_api::prelude::*;

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

	fn run(input: Ctx) {
		let _ = input;
		unimplemented!()
	}
}
