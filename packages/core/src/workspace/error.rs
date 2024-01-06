use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[repr(transparent)]
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub(crate) ErrorInner);

#[derive(Debug, Error)]
pub(crate) enum ErrorInner {
	#[error(transparent)]
	PackError(#[from] crate::pack::error::Error)
}

impl crate::util::IntoError for Error {
	type Inner = ErrorInner;
	#[inline]
	fn with_inner(inner: Self::Inner) -> Self {
		Error(inner)
	}
}
