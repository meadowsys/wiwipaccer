/// avoiding `.map_err(Into::into).map_err(Error)` and replacing it with
/// `.map_err(into)`
///
/// You're probably looking for [`into_err`]
pub(crate) trait IntoError {
	type Inner;

	fn with_inner(inner: Self::Inner) -> Self;
}

#[inline]
pub(crate) fn into_err<ETo, EFrom>(error: EFrom) -> ETo
where
	ETo: IntoError,
	ETo::Inner: From<EFrom>
{
	ETo::with_inner(error.into())
}
