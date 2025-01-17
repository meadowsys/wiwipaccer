use wiwi::prelude::*;

pub struct Model {
	// parent
	// ambient_occlusion
	// display
	// textures
	gui_light: Option<bool>
	// elements
	// overrides
	// layers
}

impl Model {
	#[inline(always)]
	pub const fn builder() -> model_builder::ModelBuilderUninit {
		model_builder::ModelBuilder::new()
	}

	#[inline(always)]
	const unsafe fn finish_init(model: MaybeUninit<Model>) -> Model {
		unsafe { model.assume_init() }
	}
}

pub mod model_builder {
	use super::*;
	use self::private::Sealed;
	use wiwi::builder::{
		Init,
		Uninit,
		IsInit,
		IsUninit,
		InitialisationStatus,
		PhantomDataInvariant
	};

	pub type ModelBuilderUninit = ModelBuilder<ModelBuilderStateContainer<Uninit>>;
	pub type ModelBuilderInit = ModelBuilder<ModelBuilderStateContainer<Init>>;

	#[repr(transparent)]
	pub struct ModelBuilder<S> {
		inner: MaybeUninit<Model>,
		__marker: PhantomDataInvariant<S>
	}

	pub trait ModelBuilderState: Sealed {
		type GuiLight: InitialisationStatus;
		type GuiLightInit: ModelBuilderState;

		type InitAll: ModelBuilderState;
	}

	pub struct ModelBuilderStateContainer<
		GuiLight
	> {
		__marker: PhantomDataInvariant<(
			GuiLight,
		)>
	}

	/// notouchie
	mod private {
		/// notouchie
		pub trait Sealed {}
	}

	impl<
		GuiLight: InitialisationStatus
	> ModelBuilderState
	for ModelBuilderStateContainer<
		GuiLight
	> {
		type GuiLight = GuiLight;
		type GuiLightInit = ModelBuilderStateContainer<Init>;

		type InitAll = ModelBuilderStateContainer<Init>;
	}

	impl<
		GuiLight: InitialisationStatus
	> Sealed
	for ModelBuilderStateContainer<
		GuiLight
	> {}

	impl ModelBuilderUninit {
		#[inline(always)]
		pub(super) const fn new() -> Self {
			Self {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	impl<S> ModelBuilder<S>
	where
		S: ModelBuilderState
	{
		#[inline(always)]
		pub const fn gui_light(mut self, gui_light: bool) -> ModelBuilder<S::GuiLightInit> {
			unsafe {
				self.gui_light_ptr().write(Some(gui_light));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn build(mut self) -> Model {
			unsafe {
				if S::GuiLight::IS_UNINIT {
					self.gui_light_ptr().write(None);
				}

				Model::finish_init(self.inner)
			}
		}

		#[inline(always)]
		const unsafe fn change_state<S2>(self) -> ModelBuilder<S2> {
			ModelBuilder {
				inner: self.inner,
				__marker: PhantomData
			}
		}

		#[inline(always)]
		const fn gui_light_ptr(&mut self) -> *mut Option<bool> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).gui_light }
		}
	}
}

pub struct Display {}

pub mod display_builder {
	use super::*;
}

pub struct Position {}

pub mod position_builder {
	use super::*;
}
