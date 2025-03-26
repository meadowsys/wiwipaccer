use self::generated::{
	Asset,
	Error,
	HostApi,
	HostAsset,
	HostError,
	HostManifestMap,
	HostRunCtx,
	HostTypes,
	ManifestKey,
	ManifestMap,
	ManifestSubItem,
	// PackEntries,
	Plugin,
	// PluginIndices,
	PluginMeta,
	// PluginPre,
	RunCtx
};
use wasmtime::{ Engine, Result, Store };
use wasmtime::component::{ Component, Linker, Resource };

mod generated;

fn _test_todo_remove_me() {
	struct State;

	impl HostApi for State {}
	impl HostTypes for State {}

	impl HostAsset for State {
		fn path(&mut self, this: Resource<Asset>) -> String {
			let _ = this;
			todo!()
		}

		fn read_all(&mut self, this: Resource<Asset>) -> Result<Vec<u8>, Resource<Error>> {
			let _ = this;
			todo!()
		}

		fn drop(&mut self, rep: Resource<Asset>) -> Result<()> {
			let _ = rep;
			// Resource::<Ctx>::new_borrow(134);
			todo!()
		}
	}

	impl HostError for State {
		fn dbg(&mut self, this: Resource<Error>) -> String {
			let _ = this;
			todo!()
		}

		fn from_str(&mut self, str: String) -> Resource<Error> {
			let _ = str;
			todo!()
		}

		fn drop(&mut self, rep: Resource<Error>) -> Result<()> {
			let _ = rep;
			todo!()
		}
	}

	impl HostManifestMap for State {
		fn get(&mut self, this: Resource<ManifestMap>, key: ManifestSubItem) -> ManifestSubItem {
			let _ = this;
			let _ = key;
			todo!()
		}

		fn drop(&mut self, rep: Resource<ManifestMap>) -> Result<()> {
			let _ = rep;
			todo!()
		}
	}

	impl HostRunCtx for State {
		fn get_assets(&mut self, this: Resource<RunCtx>) -> Result<Vec<Resource<Asset>>, Resource<Error>> {
			let _ = this;
			todo!()
		}

		fn get_manifest_key(&mut self, this: Resource<RunCtx>, key: String) -> ManifestKey {
			let _ = this;
			let _ = key;
			todo!()
		}

		fn drop(&mut self, rep: Resource<RunCtx>) -> Result<()> {
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
