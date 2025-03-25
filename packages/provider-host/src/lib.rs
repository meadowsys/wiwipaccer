use self::generated::Plugin;
use self::generated::wiwi::wiwipaccer_provider_plugin::api::Host as ApiHost;
use self::generated::wiwi::wiwipaccer_provider_plugin::types::Host as TypesHost;

use wasmtime::{ Engine, Store };
use wasmtime::component::{ Component, Linker, Resource };

mod generated {
	wasmtime::component::bindgen!({
		path: "../provider-api/provider-plugin.wit",

		// todo toy with configs if necessary
		ownership: Borrowing {
			duplicate_if_necessary: true
		}
	});
}

fn _test_todo_remove_me() {
	struct State;

	impl ApiHost for State {}
	impl TypesHost for State {}

	impl self::generated::wiwi::wiwipaccer_provider_plugin::types::HostCtxMethods for State {
		fn drop(&mut self, rep: Resource<generated::wiwi::wiwipaccer_provider_plugin::types::CtxMethods>) -> wasmtime::Result<()> {
			let _ = rep;
			unimplemented!()
		}
	}

	let engine = Engine::default();
	let component = Component::from_binary(&engine, b"Uwuwuwuwuuwuuwuuwuwuwuwuwu").unwrap();

	let mut linker = Linker::new(&engine);
	Plugin::add_to_linker::<State, _>(&mut linker, |state| state).unwrap();

	let mut store = Store::new(&engine, State);
	let _bindings = Plugin::instantiate(&mut store, &component, &linker).unwrap();
}
