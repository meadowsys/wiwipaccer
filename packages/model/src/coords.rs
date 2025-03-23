use wiwi::prelude::*;

pub struct CoordsXYZ<N> {
	x: N,
	y: N,
	z: N
}

impl<N> CoordsXYZ<N> {
	#[inline(always)]
	pub const fn builder() -> coords_xyz_builder::CoordsXYZBuilderUninit<N> {
		coords_xyz_builder::CoordsXYZBuilder::new()
	}

	#[inline(always)]
	const unsafe fn finish_init(coords: MaybeUninit<Self>) -> Self {
		unsafe { coords.assume_init() }
	}
}

pub mod coords_xyz_builder {
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

	pub type CoordsXYZBuilderUninit<N> = CoordsXYZBuilder<N, StateContainer<Uninit, Uninit, Uninit>>;
	pub type CoordsXYZBuilderInit<N> = CoordsXYZBuilder<N, StateContainer<Init, Init, Init>>;

	#[repr(transparent)]
	pub struct CoordsXYZBuilder<N, S> {
		inner: MaybeUninit<CoordsXYZ<N>>,
		__marker: PhantomDataInvariant<S>
	}

	gen_builder_state_2! {
		ident X;
		ident Y;
		ident Z;
	}

	impl<N> CoordsXYZBuilderUninit<N> {
		#[inline(always)]
		pub(super) const fn new() -> Self {
			Self {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	impl<N, S> CoordsXYZBuilder<N, S>
	where
		S: State
	{
		#[inline(always)]
		pub const fn build(self) -> CoordsXYZ<N>
		where
			S::X: IsInit,
			S::Y: IsInit,
			S::Z: IsInit
		{
			unsafe { CoordsXYZ::finish_init(self.inner) }
		}

		#[inline(always)]
		pub const fn x(mut self, x: N) -> CoordsXYZBuilder<N, S::XInit>
		where
			S::X: IsUninit
		{
			unsafe {
				self.x_ptr().write(x);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn y(mut self, y: N) -> CoordsXYZBuilder<N, S::YInit>
		where
			S::Y: IsUninit
		{
			unsafe {
				self.y_ptr().write(y);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn z(mut self, z: N) -> CoordsXYZBuilder<N, S::ZInit>
		where
			S::Z: IsUninit
		{
			unsafe {
				self.z_ptr().write(z);
				self.change_state()
			}
		}
	}

	impl<N, S> CoordsXYZBuilder<N, S> {
		#[inline(always)]
		const unsafe fn change_state<S2>(self) -> CoordsXYZBuilder<N, S2> {
			CoordsXYZBuilder {
				inner: self.inner,
				__marker: PhantomData
			}
		}

		#[inline(always)]
		const fn x_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).x }
		}

		#[inline(always)]
		const fn y_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).y }
		}

		#[inline(always)]
		const fn z_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).z }
		}
	}
}

pub struct CoordsUV<N> {
	u: N,
	v: N
}

impl<N> CoordsUV<N> {
	#[inline(always)]
	pub const fn builder() -> coords_uv_builder::CoordsUVBuilderUninit<N> {
		coords_uv_builder::CoordsUVBuilder::new()
	}

	#[inline(always)]
	const unsafe fn finish_init(coords: MaybeUninit<Self>) -> Self {
		unsafe { coords.assume_init() }
	}
}

pub mod coords_uv_builder {
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

	pub type CoordsUVBuilderUninit<N> = CoordsUVBuilder<N, StateContainer<Uninit, Uninit>>;
	pub type CoordsUVBuilderInit<N> = CoordsUVBuilder<N, StateContainer<Init, Init>>;

	pub struct CoordsUVBuilder<N, S> {
		inner: MaybeUninit<CoordsUV<N>>,
		__marker: PhantomDataInvariant<S>
	}

	gen_builder_state_2! {
		ident U;
		ident V;
	}

	impl<N> CoordsUVBuilderUninit<N> {
		#[inline(always)]
		pub(super) const fn new() -> Self {
			Self {
				inner: MaybeUninit::uninit(),
				__marker: PhantomData
			}
		}
	}

	impl<N, S> CoordsUVBuilder<N, S>
	where
		S: State
	{
		#[inline(always)]
		pub const fn build(self) -> CoordsUV<N>
		where
			S::U: IsInit,
			S::V: IsInit
		{
			unsafe { CoordsUV::finish_init(self.inner) }
		}

		#[inline(always)]
		pub const fn u(mut self, u: N) -> CoordsUVBuilder<N, S::UInit>
		where
			S::U: IsUninit
		{
			unsafe {
				self.u_ptr().write(u);
				self.change_state()
			}
		}

		#[inline(always)]
		pub const fn v(mut self, v: N) -> CoordsUVBuilder<N, S::VInit>
		where
			S::V: IsUninit
		{
			unsafe {
				self.v_ptr().write(v);
				self.change_state()
			}
		}
	}

	impl<N, S> CoordsUVBuilder<N, S> {
		#[inline(always)]
		const unsafe fn change_state<S2>(self) -> CoordsUVBuilder<N, S2> {
			CoordsUVBuilder {
				inner: self.inner,
				__marker: PhantomData
			}
		}

		#[inline(always)]
		const fn u_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).u }
		}

		#[inline(always)]
		const fn v_ptr(&mut self) -> *mut N {
			unsafe { &raw mut (*self.inner.as_mut_ptr()).v }
		}
	}
}
