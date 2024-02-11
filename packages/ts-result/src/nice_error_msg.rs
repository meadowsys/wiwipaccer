use crate::Formatter;
use ::std::fmt;

pub trait NiceErrorMessage {
	/// # Correctness
	///
	/// This function is expected to write its message with no leading or
	/// trailing newlines. Calling [`Formatter::next_line`] in the middle of
	/// other non-whitespace characters is okay and correct, but writing a
	/// newline character or strings with newline characters is not. You also
	/// should not call [`Formatter::next_line`] at the start or end of the
	/// function, before/after anything else.
	fn fmt(&self, f: &mut Formatter);

	#[inline]
	fn to_error_message(&self) -> String {
		let mut formatter = Formatter::new();
		self.fmt(&mut formatter);
		formatter.into_string()
	}
}

impl<T: ?Sized + NiceErrorMessage> NiceErrorMessage for &T {
	#[inline]
	fn fmt(&self, f: &mut Formatter) {
		<T as NiceErrorMessage>::fmt(self, f);
	}
}

impl<T: ?Sized + NiceErrorMessage> NiceErrorMessage for &mut T {
	#[inline]
	fn fmt(&self, f: &mut Formatter) {
		<T as NiceErrorMessage>::fmt(self, f);
	}
}

impl<T: NiceErrorMessage> NiceErrorMessage for Vec<T> {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("multiple errors:");
		f.with_indent(|f| for item in self {
			f.fmt(item);
		});
	}
}


pub trait NiceErrorMessageExt: NiceErrorMessage {
	#[inline]
	fn as_display(&self) -> NiceErrorMessageDisplay<'_, Self> {
		NiceErrorMessageDisplay { inner: self }
	}
}

impl<T: NiceErrorMessage> NiceErrorMessageExt for T {}

#[repr(transparent)]
pub struct NiceErrorMessageDisplay<'h, T: ?Sized> {
	inner: &'h T
}

impl<'h, T: NiceErrorMessage> fmt::Display for NiceErrorMessageDisplay<'h, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO optimisation to use our formatter stuff?
		f.write_str(&self.inner.to_error_message())
	}
}
