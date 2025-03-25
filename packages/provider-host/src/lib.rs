use self::generated::Plugin;
use self::generated::wiwi::wiwipaccer_provider_plugin::api::Host as ApiHost;
use self::generated::wiwi::wiwipaccer_provider_plugin::types::{ Asset, Ctx, HostAsset, HostCtx, Host as TypesHost };
use wasmtime::{ Engine, Result, Store };
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

	impl HostAsset for State {
		fn path(&mut self, this: Resource<Asset>) -> String {
			let _ = this;
			todo!()
		}

		fn drop(&mut self, rep: Resource<Asset>) -> Result<()> {
			let _ = rep;
			todo!()
		}
	}

	impl HostCtx for State {
		fn get_assets(&mut self, this: Resource<Ctx>) -> Vec<Resource<Asset>> {
			let _ = this;
			todo!()
		}

		fn drop(&mut self, rep: Resource<Ctx>) -> Result<()> {
			let _ = rep;
			todo!()
		}
	}

	let engine = Engine::default();
	let component = Component::from_binary(&engine, b"Uwuwuwuwuuwuuwuuwuwuwuwuwu").unwrap();

	let mut linker = Linker::new(&engine);
	Plugin::add_to_linker::<State, _>(&mut linker, |state| state).unwrap();

	let mut store = Store::new(&engine, State);
	let _bindings = Plugin::instantiate(&mut store, &component, &linker).unwrap();
}
