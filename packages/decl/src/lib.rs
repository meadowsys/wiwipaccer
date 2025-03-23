#![allow(
	dead_code,
	unused_imports,
	reason = "wip (todo remove me)"
)]

use wiwi::prelude::*;

pub struct Texture {
	id: Box<str>,
	name: Box<str>,
	block_id: Box<str>
	// blockstates
}

// - todo impl struct with `builder()` and `finish_init(..)`
impl Texture {
	pub fn builder() -> texture_builder::TextureBuilderUninit {
		texture_builder::TextureBuilder::new()
	}
}

pub mod texture_builder {
	use super::*;
	use wiwi::builder::{
		Init,
		Uninit,
		IsInit,
		IsUninit,
		InitStatus,
		PhantomDataInvariant,
		gen_builder_state_2
	};

	pub type TextureBuilderInit = TextureBuilder<StateContainer<Init, Init, Init>>;
	pub type TextureBuilderUninit = TextureBuilder<StateContainer<Uninit, Uninit, Uninit>>;

	// - todo builder struct def
	pub struct TextureBuilder<S> {
		inner: MaybeUninit<Texture>,
		__marker: PhantomDataInvariant<S>
	}

	gen_builder_state_2! {
		ident Id;
		ident Name;
		ident BlockId;
	}

	// - todo impl uninit for `new()` fn
	impl TextureBuilderUninit {
		pub(super) const fn new() -> Self {
			Self {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	// - todo impl<S> where S: builder state trait for all the fns including `build()`
	//   (`build()` calls `finish_init(..)`)

	// - todo impl block, same as previous one in headers and stuffs, for the internal fns
}

// fn validate_id(id: &str) -> bool { todo!() }

// fn validate_name(name: &str) -> bool { todo!() }

// fn validate_block_id(block_id: &str) -> bool { todo!() }

// todo errors for validation fns above ^
// ex. error for invalid char or something
