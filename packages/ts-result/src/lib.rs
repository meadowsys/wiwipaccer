//! custom error type to avoid promise rejecting, and present an API similar to
//! [`Result`] in rust, by representing it as a union type

pub mod formatter;

pub use self::formatter::Formatter;
use ::serde::{ Serialize, Serializer };
use ::serde::ser::SerializeStruct;
use ::std::convert::Infallible;
use ::std::fmt;

pub use ::std::error::Error;

pub enum TSResult<T, E> {
	Ok(T),
	Err(E)
}

pub use TSResult::{ Ok, Err };

impl<T, E> From<Result<T, E>> for TSResult<T, E> {
	#[inline]
	fn from(value: Result<T, E>) -> Self {
		match value {
			Result::Ok(v) => { Ok(v) }
			Result::Err(e) => { Err(e) }
		}
	}
}

impl<T, E> From<TSResult<T, E>> for Result<T, E> {
	#[inline]
	fn from(value: TSResult<T, E>) -> Self {
		match value {
			Ok(v) => { Result::Ok(v) }
			Err(e) => { Result::Err(e) }
		}
	}
}

impl<T, E> Serialize for TSResult<T, E>
where
	T: Serialize,
	E: NiceErrorMessage
{
	#[inline]
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		const SUCCESS: &str = "success";
		const VALUE: &str = "value";
		const ERROR: &str = "error";

		let mut s = s.serialize_struct("TSResult", 2)?;

		match self {
			Ok(val) => {
				s.serialize_field(SUCCESS, &true)?;
				s.serialize_field(VALUE, val)?;
			}
			Err(err) => {
				s.serialize_field(SUCCESS, &false)?;
				s.serialize_field(ERROR, &err.to_error_message())?;
			}
		}

		s.end()
	}
}

pub type WrappedTSResult<T, E, RE = Infallible> = Result<TSResult<T, E>, RE>;

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

impl<'h, T> fmt::Display for NiceErrorMessageDisplay<'h, T> where T: NiceErrorMessage {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO optimisation to use our formatter stuff?
		f.write_str(&self.inner.to_error_message())
	}
}

pub trait WrapInTSResult<T, E> {
	fn wrap_in_ts_result<RE>(self) -> WrappedTSResult<T, E, RE>;
}

impl<T, E> WrapInTSResult<T, E> for Result<T, E> {
	fn wrap_in_ts_result<RE>(self) -> WrappedTSResult<T, E, RE> {
		match self {
			Result::Ok(v) => { Result::Ok(Ok(v)) }
			Result::Err(e) => { Result::Ok(Err(e)) }
		}
	}
}

#[macro_export]
macro_rules! impl_display {
	($struct:ident) => {
		impl ::std::fmt::Display for $struct {
			#[inline]
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				f.write_str(&self.to_error_message())
			}
		}
	}
}

#[inline]
pub fn lines(lines: &[String]) -> String {
	lines.join("\n")
}
