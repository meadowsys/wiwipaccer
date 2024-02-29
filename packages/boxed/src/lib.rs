pub trait Boxed {
	fn boxed(self) -> Box<Self>;
}

impl<T> Boxed for T {
	#[inline(always)]
	fn boxed(self) -> Box<Self> {
		Box::new(self)
	}
}
