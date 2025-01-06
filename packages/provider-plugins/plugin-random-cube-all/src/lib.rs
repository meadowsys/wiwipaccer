// wit_bindgen::generate! {{
// 	path: "../plugin.wit",
// 	ownership: Borrowing { duplicate_if_necessary: true }
// }}

// use exports::wiwi::wiwipaccer_provider_plugin::imp;
// use wiwi::wiwipaccer_provider_plugin::api;

// struct Plugin;
// export!(Plugin);

// impl imp::Guest for Plugin {
// 	fn metadata() -> imp::PluginMetadata {
// 		imp::PluginMetadata {
// 			ns: "wiwi".into(),
// 			id: "random-cube-all".into(),
// 			version: "0.1.0-testlol".into(),
// 			authors: vec![
// 				"Meadow Liu <meadowsys@kiwin.gay>".into(),
// 				api::h()
// 			]
// 		}
// 	}
// }

pub const fn å() -> &'static str {
	"aaaaaaaaaaaaaaaaaa"
}
