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

impl Display {
	#[inline(always)]
	pub const fn builder() -> display_builder::DisplayBuilderUninit {
		display_builder::DisplayBuilder::new()
	}

	#[inline(always)]
	const unsafe fn finish_init(display: MaybeUninit<Self>) -> Self {
		unsafe { display.assume_init() }
	}
}
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

	pub type DisplayBuilderUninit = DisplayBuilder<StateContainer<Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit>>;
	pub type DisplayBuilderInit = DisplayBuilder<StateContainer<Init, Init, Init, Init, Init, Init, Init, Init>>;

	pub struct DisplayBuilder<S> {
		inner: MaybeUninit<Display>,
		__marker: PhantomDataInvariant<S>
	}

	pub trait State {
		type FirstLeft: InitialisationStatus;
		type FirstLeftInit: State;

		type FirstRight: InitialisationStatus;
		type FirstRightInit: State;

		type ThirdLeft: InitialisationStatus;
		type ThirdLeftInit: State;

		type ThirdRight: InitialisationStatus;
		type ThirdRightInit: State;

		type Gui: InitialisationStatus;
		type GuiInit: State;

		type Head: InitialisationStatus;
		type HeadInit: State;

		type Ground: InitialisationStatus;
		type GroundInit: State;

		type Fixed: InitialisationStatus;
		type FixedInit: State;
	}

	#[expect(clippy::type_complexity, reason = "shut")]
	pub struct StateContainer<
		FirstLeft,
		FirstRight,
		ThirdLeft,
		ThirdRight,
		Gui,
		Head,
		Ground,
		Fixed
	> {
		__marker: PhantomDataInvariant<(
			FirstLeft,
			FirstRight,
			ThirdLeft,
			ThirdRight,
			Gui,
			Head,
			Ground,
			Fixed
		)>
	}

	/// notouchie
	mod private {
		/// notouchie
		pub trait Sealed {}
	}

	impl<
		FirstLeft: InitialisationStatus,
		FirstRight: InitialisationStatus,
		ThirdLeft: InitialisationStatus,
		ThirdRight: InitialisationStatus,
		Gui: InitialisationStatus,
		Head: InitialisationStatus,
		Ground: InitialisationStatus,
		Fixed: InitialisationStatus
	> State for StateContainer<
		FirstLeft,
		FirstRight,
		ThirdLeft,
		ThirdRight,
		Gui,
		Head,
		Ground,
		Fixed
	> {
		type FirstLeft = FirstLeft;
		type FirstLeftInit = StateContainer<Init, FirstRight, ThirdLeft, ThirdRight, Gui, Head, Ground, Fixed>;

		type FirstRight = FirstRight;
		type FirstRightInit = StateContainer<FirstLeft, Init, ThirdLeft, ThirdRight, Gui, Head, Ground, Fixed>;

		type ThirdLeft = ThirdLeft;
		type ThirdLeftInit = StateContainer<FirstLeft, FirstRight, Init, ThirdRight, Gui, Head, Ground, Fixed>;

		type ThirdRight = ThirdRight;
		type ThirdRightInit = StateContainer<FirstLeft, FirstRight, ThirdLeft, Init, Gui, Head, Ground, Fixed>;

		type Gui = Gui;
		type GuiInit = StateContainer<FirstLeft, FirstRight, ThirdLeft, ThirdRight, Init, Head, Ground, Fixed>;

		type Head = Head;
		type HeadInit = StateContainer<FirstLeft, FirstRight, ThirdLeft, ThirdRight, Gui, Init, Ground, Fixed>;

		type Ground = Ground;
		type GroundInit = StateContainer<FirstLeft, FirstRight, ThirdLeft, ThirdRight, Gui, Head, Init, Fixed>;

		type Fixed = Fixed;
		type FixedInit = StateContainer<FirstLeft, FirstRight, ThirdLeft, ThirdRight, Gui, Head, Ground, Init>;
	}

	impl<
		FirstLeft: InitialisationStatus,
		FirstRight: InitialisationStatus,
		ThirdLeft: InitialisationStatus,
		ThirdRight: InitialisationStatus,
		Gui: InitialisationStatus,
		Head: InitialisationStatus,
		Ground: InitialisationStatus,
		Fixed: InitialisationStatus
	> Sealed for StateContainer<
		FirstLeft,
		FirstRight,
		ThirdLeft,
		ThirdRight,
		Gui,
		Head,
		Ground,
		Fixed
	> {}

	impl DisplayBuilderUninit {
		#[inline(always)]
		pub(super) const fn new() -> Self {
			DisplayBuilder {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	impl<S> DisplayBuilder<S>
	where
		S: State
	{
		#[inline(always)]
		pub const fn build(mut self) -> Display {
			if S::FirstLeft::IS_UNINIT {
				unsafe { self.first_left_ptr().write(None) }
			}

			if S::FirstRight::IS_UNINIT {
				unsafe { self.first_right_ptr().write(None) }
			}

			if S::ThirdLeft::IS_UNINIT {
				unsafe { self.third_left_ptr().write(None) }
			}

			if S::ThirdRight::IS_UNINIT {
				unsafe { self.third_right_ptr().write(None) }
			}

			if S::Gui::IS_UNINIT {
				unsafe { self.gui_ptr().write(None) }
			}

			if S::Head::IS_UNINIT {
				unsafe { self.head_ptr().write(None) }
			}

			if S::Ground::IS_UNINIT {
				unsafe { self.ground_ptr().write(None) }
			}

			if S::Fixed::IS_UNINIT {
				unsafe { self.fixed_ptr().write(None) }
			}

			unsafe { Display::finish_init(self.inner) }
		}

		#[inline(always)]
		pub const fn first_left(
			mut self,
			first_left: Position
		) -> DisplayBuilder<S::FirstLeftInit>
		where
			S::FirstLeft: IsUninit
		{
			unsafe {
				self.first_left_ptr().write(Some(first_left));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn first_right(
			mut self,
			first_right: Position
		) -> DisplayBuilder<S::FirstRightInit>
		where
			S::FirstRight: IsUninit
		{
			unsafe {
				self.first_right_ptr().write(Some(first_right));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn third_left(
			mut self,
			third_left: Position
		) -> DisplayBuilder<S::ThirdLeftInit>
		where
			S::ThirdLeft: IsUninit
		{
			unsafe {
				self.third_left_ptr().write(Some(third_left));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn third_right(
			mut self,
			third_right: Position
		) -> DisplayBuilder<S::ThirdRightInit>
		where
			S::ThirdRight: IsUninit
		{
			unsafe {
				self.third_right_ptr().write(Some(third_right));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn gui(
			mut self,
			gui: Position
		) -> DisplayBuilder<S::GuiInit>
		where
			S::Gui: IsUninit
		{
			unsafe {
				self.gui_ptr().write(Some(gui));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn head(
			mut self,
			head: Position
		) -> DisplayBuilder<S::HeadInit>
		where
			S::Head: IsUninit
		{
			unsafe {
				self.head_ptr().write(Some(head));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn ground(
			mut self,
			ground: Position
		) -> DisplayBuilder<S::GroundInit>
		where
			S::Ground: IsUninit
		{
			unsafe {
				self.ground_ptr().write(Some(ground));
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn fixed(
			mut self,
			fixed: Position
		) -> DisplayBuilder<S::FixedInit>
		where
			S::Fixed: IsUninit
		{
			unsafe {
				self.fixed_ptr().write(Some(fixed));
				self.change_state()
			}
		}
	}

	impl<S> DisplayBuilder<S> {
		#[inline(always)]
		const unsafe fn change_state<S2>(self) -> DisplayBuilder<S2> {
			DisplayBuilder {
				inner: self.inner,
				__marker: PhantomData
			}
		}

		#[inline(always)]
		const fn first_left_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).first_left }
		}

		#[inline(always)]
		const fn first_right_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).first_right }
		}

		#[inline(always)]
		const fn third_left_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).third_left }
		}

		#[inline(always)]
		const fn third_right_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).third_right }
		}

		#[inline(always)]
		const fn gui_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).gui }
		}

		#[inline(always)]
		const fn head_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).head }
		}

		#[inline(always)]
		const fn ground_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).ground }
		}

		#[inline(always)]
		const fn fixed_ptr(&mut self) -> *mut Option<Position> {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).fixed }
		}
	}
}

pub struct Position {
	rotation: CoordsXYZ<f64>,
	translation: CoordsXYZ<f64>,
	scale: CoordsXYZ<f64>
}

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
		pub const fn rotation(
			mut self,
			rotation: CoordsXYZ<f64>
		) -> PositionBuilder<S::RotationInit> {
			unsafe {
				self.rotation_ptr().write(rotation);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn translation(
			mut self,
			translation: CoordsXYZ<f64>
		) -> PositionBuilder<S::TranslationInit> {
			unsafe {
				self.translation_ptr().write(translation);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn scale(
			mut self,
			scale: CoordsXYZ<f64>
		) -> PositionBuilder<S::ScaleInit> {
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
