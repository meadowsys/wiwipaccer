use wiwi::prelude::*;

pub struct CoordsXYZ<N> {
	x: N,
	y: N,
	z: N
}

impl<N> CoordsXYZ<N> {
	#[inline(always)]
	pub fn builder() -> coords_xyz_builder::CoordsXYZBuilderUninit<N> {
		coords_xyz_builder::CoordsXYZBuilder::new()
	}

	#[inline(always)]
	unsafe fn finish_init(coords: MaybeUninit<CoordsXYZ<N>>) -> CoordsXYZ<N> {
		unsafe { coords.assume_init() }
	}
}

pub mod coords_xyz_builder {
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

	pub type CoordsXYZBuilderUninit<N> = CoordsXYZBuilder<N, CoordsXYZBuilderStateContainer<Uninit, Uninit, Uninit>>;
	pub type CoordsXYZBuilderInit<N> = CoordsXYZBuilder<N, CoordsXYZBuilderStateContainer<Init, Init, Init>>;

	#[repr(transparent)]
	pub struct CoordsXYZBuilder<N, S> {
		inner: MaybeUninit<CoordsXYZ<N>>,
		__marker: PhantomDataInvariant<S>
	}

	pub trait CoordsXYZBuilderState: Sealed {
		type X: InitialisationStatus;
		type XInit: CoordsXYZBuilderState;

		type Y: InitialisationStatus;
		type YInit: CoordsXYZBuilderState;

		type Z: InitialisationStatus;
		type ZInit: CoordsXYZBuilderState;

		type InitAll: CoordsXYZBuilderState;
	}

	pub struct CoordsXYZBuilderStateContainer<X, Y, Z> {
		__marker: PhantomDataInvariant<(X, Y, Z)>
	}

	/// notouchie
	mod private {
		/// notouchie
		pub trait Sealed {}
	}

	impl<
		X: InitialisationStatus,
		Y: InitialisationStatus,
		Z: InitialisationStatus
	> CoordsXYZBuilderState for CoordsXYZBuilderStateContainer<
		X,
		Y,
		Z
	> {
		type X = X;
		type XInit = CoordsXYZBuilderStateContainer<Init, Y, Z>;

		type Y = Y;
		type YInit = CoordsXYZBuilderStateContainer<X, Init, Z>;

		type Z = Z;
		type ZInit = CoordsXYZBuilderStateContainer<X, Y, Init>;

		type InitAll = CoordsXYZBuilderStateContainer<Init, Init, Init>;
	}

	impl<
		X: InitialisationStatus,
		Y: InitialisationStatus,
		Z: InitialisationStatus
	> Sealed for CoordsXYZBuilderStateContainer<
		X,
		Y,
		Z
	> {}

	impl<N> CoordsXYZBuilderUninit<N> {
		#[inline(always)]
		pub(super) fn new() -> Self {
			Self {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	impl<N, S> CoordsXYZBuilder<N, S>
	where
		S: CoordsXYZBuilderState
	{
		#[inline(always)]
		pub fn x(mut self, x: N) -> CoordsXYZBuilder<N, S::XInit> {
			unsafe {
				self.x_ptr().write(x);
				self.change_state()
			}
		}

		#[inline(always)]
		pub fn y(mut self, y: N) -> CoordsXYZBuilder<N, S::YInit> {
			unsafe {
				self.y_ptr().write(y);
				self.change_state()
			}
		}

		#[inline(always)]
		pub fn z(mut self, z: N) -> CoordsXYZBuilder<N, S::ZInit> {
			unsafe {
				self.z_ptr().write(z);
				self.change_state()
			}
		}

		#[inline(always)]
		pub fn build(self) -> CoordsXYZ<N>
		where
			S::X: IsInit,
			S::Y: IsInit,
			S::Z: IsInit
		{
			unsafe { CoordsXYZ::finish_init(self.inner) }
		}

		#[inline(always)]
		unsafe fn change_state<S2>(self) -> CoordsXYZBuilder<N, S2> {
			CoordsXYZBuilder {
				inner: self.inner,
				__marker: PhantomData
			}
		}

		#[inline(always)]
		fn x_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).x }
		}

		#[inline(always)]
		fn y_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).y }
		}

		#[inline(always)]
		fn z_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).z }
		}
	}
}

pub struct CoordsUV<N> {
	u: N,
	v: N
}
