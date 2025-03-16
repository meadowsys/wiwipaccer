wasmtime::component::bindgen!({
	path: "../provider-plugin-api/provider-plugin.wit",

	// ???
	// async: true,
	// or configure it more finely idk
	ownership: Borrowing {
		duplicate_if_necessary: true
	}
});

fn _test_todo_remove_me() {
	use wasmtime::component::{ Component, Linker };
	use wasmtime::{ Engine, Store };

	struct State;

	impl self::wiwi::wiwipaccer_provider_plugin::api::Host for State {}
	impl self::wiwi::wiwipaccer_provider_plugin::types::Host for State {}

	let engine = Engine::default();
	let component = Component::from_binary(&engine, b"Uwuwuwuwuuwuuwuuwuwuwuwuwu").unwrap();

	let mut linker = Linker::new(&engine);
	Plugin::add_to_linker::<State, _>(&mut linker, |state| state).unwrap();

	let mut store = Store::new(&engine, State);
	let _bindings = Plugin::instantiate(&mut store, &component, &linker).unwrap();
}
