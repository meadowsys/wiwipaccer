use self::generated::{ Plugin, PluginMeta, PluginPre };
use self::state::State;
use wasmtime::{ Engine, Store };
use wasmtime::component::{ Component, Linker };

mod generated;
mod state;

fn _test_todo_remove_me() {
	let engine = Engine::default();
	let component = Component::from_binary(
		&engine,
		b"pretend this is the bytes of a wasm component"
	).unwrap();

	let mut linker = Linker::new(&engine);
	Plugin::add_to_linker::<State, _>(&mut linker, |s| s).unwrap();

	let mut store = Store::new(&engine, State::new());

	let pre = linker.instantiate_pre(&component).unwrap();
	let plugin_pre = PluginPre::new(pre).unwrap();
	let _bindings = plugin_pre.instantiate(&mut store).unwrap();

	let PluginMeta {
		ns: _ns,
		id: _id,
		version: _version
	} = _bindings.wiwi_wiwipaccer_provider_plugin_imp().call_meta(&mut store).unwrap();
}
