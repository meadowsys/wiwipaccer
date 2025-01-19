use wiwi::prelude::*;

use crate::coords::CoordsXYZ;
use hashbrown::HashMap;
use wiwi::num::*;

pub struct Model {
	// parent: Box<str>,
	// ambient_occlusion: bool,
	// display: Display,
	// textures: Textures,
	// gui_light: GuiLight,
	// elements: Vec<Element>
	// overrides
	// layers
}

pub struct Display {
	first_left: Option<Position>,
	first_right: Option<Position>,
	third_left: Option<Position>,
	third_right: Option<Position>,
	gui: Option<Position>,
	head: Option<Position>,
	ground: Option<Position>,
	fixed: Option<Position>
}

// - TODO: impl struct with `builder()` and `finish_init(..)`

pub mod display_builder {
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
	//   - TODO: pub type for init/uninit
	pub struct DisplayBuilder<S> {
		inner: MaybeUninit<Display>,
		__marker: PhantomDataInvariant<S>
	}
	//   - TODO: builder state trait def
	//   - TODO: builder state container struct def

	/// notouchie
	mod private {
		/// notouchie
		pub trait Sealed {}
	}

	//   - TODO: impl builder state trait
	//   - TODO: impl sealed
	//   - TODO: impl uninit for `new()` fn
	//   - TODO: impl<S> where S: builder state trait for all the fns including `build()`
	//     (`build()` calls `finish_init(..)`)
	//   - TODO: impl block, same as previous one in headers and stuffs, for the internal fns
}

pub struct Position {
	rotation: CoordsXYZ<f64>,
	translation: CoordsXYZ<f64>,
	scale: CoordsXYZ<f64>
}

// - TODO: impl struct with `builder()` and `finish_init(..)`
impl Position {
	#[inline(always)]
	pub const fn builder() -> position_builder::PositionBuilderUninit {
		position_builder::PositionBuilder::new()
	}

	#[inline(always)]
	const unsafe fn finish_init(position: MaybeUninit<Self>) -> Self {
		unsafe { position.assume_init() }
	}
}

pub mod position_builder {
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

	pub type PositionBuilderUninit = PositionBuilder<StateContainer<Uninit, Uninit, Uninit>>;
	pub type PositionBuilderInit = PositionBuilder<StateContainer<Init, Init, Init>>;

	pub struct PositionBuilder<S> {
		inner: MaybeUninit<Position>,
		__marker: PhantomDataInvariant<S>
	}

	pub trait State: Sealed {
		type Rotation: InitialisationStatus;
		type RotationInit: State;

		type Translation: InitialisationStatus;
		type TranslationInit: State;

		type Scale: InitialisationStatus;
		type ScaleInit: State;
	}

	pub struct StateContainer<Rotation, Translation, Scale> {
		__marker: PhantomDataInvariant<(Rotation, Translation, Scale)>
	}

	/// notouchie
	mod private {
		/// notouchie
		pub trait Sealed {}
	}

	impl<
		Rotation: InitialisationStatus,
		Translation: InitialisationStatus,
		Scale: InitialisationStatus
	> State for StateContainer<Rotation, Translation, Scale> {
		type Rotation = Rotation;
		type RotationInit = StateContainer<Init, Translation, Scale>;

		type Translation = Translation;
		type TranslationInit = StateContainer<Rotation, Init, Scale>;

		type Scale = Scale;
		type ScaleInit = StateContainer<Rotation, Translation, Init>;
	}

	impl<
		Rotation: InitialisationStatus,
		Translation: InitialisationStatus,
		Scale: InitialisationStatus
	> Sealed for StateContainer<Rotation, Translation, Scale> {}

	impl PositionBuilderUninit {
		#[inline(always)]
		pub(super) const fn new() -> Self {
			Self {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	impl<S> PositionBuilder<S>
	where
		S: State
	{
		#[inline(always)]
		pub const fn build(self) -> Position
		where
			S::Rotation: IsInit,
			S::Translation: IsInit,
			S::Scale: IsInit
		{
			unsafe { Position::finish_init(self.inner) }
		}


		#[inline(always)]
		pub const fn rotation(mut self, rotation: CoordsXYZ<f64>) -> PositionBuilder<S::RotationInit> {
			unsafe {
				self.rotation_ptr().write(rotation);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn translation(mut self, translation: CoordsXYZ<f64>) -> PositionBuilder<S::TranslationInit> {
			unsafe {
				self.translation_ptr().write(translation);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn scale(mut self, scale: CoordsXYZ<f64>) -> PositionBuilder<S::ScaleInit> {
			unsafe {
				self.scale_ptr().write(scale);
				self.change_state()
			}
		}
	}

	impl<S> PositionBuilder<S> {
		#[inline(always)]
		const unsafe fn change_state<S2>(self) -> PositionBuilder<S2> {
			PositionBuilder {
				inner: self.inner,
				__marker: PhantomData
			}
		}

		#[inline(always)]
		const fn rotation_ptr(&mut self) -> *mut CoordsXYZ<f64> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).rotation }
		}

		#[inline(always)]
		const fn translation_ptr(&mut self) -> *mut CoordsXYZ<f64> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).translation }
		}

		#[inline(always)]
		const fn scale_ptr(&mut self) -> *mut CoordsXYZ<f64> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).scale }
		}
	}
}
