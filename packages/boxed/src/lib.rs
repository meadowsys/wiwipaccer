pub trait Boxed {
	/// Wraps the value into a box, storing it onto the heap.
	fn boxed(self) -> Box<Self>;
}

impl<T> Boxed for T {
	#[inline(always)]
	fn boxed(self) -> Box<Self> {
		Box::new(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_boxes() {
		let value = 3900238.boxed();
		assert_eq!(3900238, *value);
	}
}
