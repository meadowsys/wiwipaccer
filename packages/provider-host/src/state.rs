use crate::generated::{
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
	RunCtx
};
use wasmtime::Result;
use wasmtime::component::Resource;

pub struct State {
	asset_state: AssetState,
	error_state: ErrorState,
	manifest_map_state: ManifestMapState,
	run_ctx_state: RunCtxState
}

impl State {
	pub fn new() -> Self {
		let asset_state = AssetState {};
		let error_state = ErrorState {};
		let manifest_map_state = ManifestMapState {};
		let run_ctx_state = RunCtxState {};

		Self {
			asset_state,
			error_state,
			manifest_map_state,
			run_ctx_state
		}
	}
}

impl HostApi for State {}
impl HostTypes for State {}

impl HostAsset for State {
	#[inline]
	fn path(&mut self, this: Resource<Asset>) -> String {
		self.asset_state.path(this)
	}

	#[inline]
	fn read_all(&mut self, this: Resource<Asset>) -> Result<Vec<u8>, Resource<Error>> {
		self.asset_state.read_all(this)
	}

	#[inline]
	fn drop(&mut self, rep: Resource<Asset>) -> Result<()> {
		self.asset_state.drop(rep)
	}
}

impl HostError for State {
	#[inline]
	fn dbg(&mut self, this: Resource<Error>) -> String {
		self.error_state.dbg(this)
	}

	#[inline]
	fn from_str(&mut self, str: String) -> Resource<Error> {
		self.error_state.from_str(str)
	}

	#[inline]
	fn drop(&mut self, rep: Resource<Error>) -> Result<()> {
		self.error_state.drop(rep)
	}
}

impl HostManifestMap for State {
	#[inline]
	fn get(&mut self, this: Resource<ManifestMap>, key: ManifestSubItem) -> ManifestSubItem {
		self.manifest_map_state.get(this, key)
	}

	#[inline]
	fn drop(&mut self, rep: Resource<ManifestMap>) -> Result<()> {
		self.manifest_map_state.drop(rep)
	}
}

impl HostRunCtx for State {
	#[inline]
	fn get_assets(&mut self, this: Resource<RunCtx>) -> Result<Vec<Resource<Asset>>, Resource<Error>> {
		self.run_ctx_state.get_assets(this)
	}

	#[inline]
	fn get_manifest_key(&mut self, this: Resource<RunCtx>, key: String) -> ManifestKey {
		self.run_ctx_state.get_manifest_key(this, key)
	}

	#[inline]
	fn drop(&mut self, rep: Resource<RunCtx>) -> Result<()> {
		self.run_ctx_state.drop(rep)
	}
}

pub struct AssetState {}

impl HostAsset for AssetState {
	#[inline]
	fn path(&mut self, this: Resource<Asset>) -> String {
		let _ = this;
		todo!()
	}

	#[inline]
	fn read_all(&mut self, this: Resource<Asset>) -> Result<Vec<u8>, Resource<Error>> {
		let _ = this;
		todo!()
	}

	#[inline]
	fn drop(&mut self, rep: Resource<Asset>) -> Result<()> {
		let _ = rep;
		todo!()
	}
}

pub struct ErrorState {}

impl HostError for ErrorState {
	#[inline]
	fn dbg(&mut self, this: Resource<Error>) -> String {
		let _ = this;
		todo!()
	}

	#[inline]
	fn from_str(&mut self, str: String) -> Resource<Error> {
		let _ = str;
		todo!()
	}

	#[inline]
	fn drop(&mut self, rep: Resource<Error>) -> Result<()> {
		let _ = rep;
		todo!()
	}
}

pub struct ManifestMapState {}

impl HostManifestMap for ManifestMapState {
	#[inline]
	fn get(&mut self, this: Resource<ManifestMap>, key: ManifestSubItem) -> ManifestSubItem {
		let _ = this;
		let _ = key;
		todo!()
	}

	#[inline]
	fn drop(&mut self, rep: Resource<ManifestMap>) -> Result<()> {
		let _ = rep;
		todo!()
	}
}

pub struct RunCtxState {}

impl HostRunCtx for RunCtxState {
	#[inline]
	fn get_assets(&mut self, this: Resource<RunCtx>) -> Result<Vec<Resource<Asset>>, Resource<Error>> {
		let _ = this;
		todo!()
	}

	#[inline]
	fn get_manifest_key(&mut self, this: Resource<RunCtx>, key: String) -> ManifestKey {
		let _ = this;
		let _ = key;
		todo!()
	}

	#[inline]
	fn drop(&mut self, rep: Resource<RunCtx>) -> Result<()> {
		let _ = rep;
		todo!()
	}
}
