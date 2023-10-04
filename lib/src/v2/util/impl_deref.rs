macro_rules! impl_deref {
	(*$type:ty => $target:ty) => {
		impl std::ops::Deref for $type {
			type Target = $target;
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	}
}

pub(crate) use impl_deref;
