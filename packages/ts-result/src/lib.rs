//! custom error type to avoid promise rejecting, and present an API similar to
//! [`Result`] in rust, by representing it as a union type

use ::serde::{ Serialize, Serializer };
use ::serde::ser::SerializeStruct;
use ::std::convert::Infallible;
use ::std::future::Future;

pub enum TSResult<T, E> {
	Ok(T),
	Err(E)
}

pub use TSResult::{ Ok, Err };

pub trait NiceErrorMessage {
	fn to_error_message(&self) -> String;
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

#[inline]
pub fn wrapped_ts_result<T, E, RE>(result: Result<T, E>)
	-> WrappedTSResult<T, E, RE>
{
	match result {
		Result::Ok(v) => { Result::Ok(Ok(v)) }
		Result::Err(e) => { Result::Ok(Err(e)) }
	}
}

#[inline]
pub fn wrapped_ts_result_fn<T, E, RE, F>(f: F)
	-> WrappedTSResult<T, E, RE>
where
	F: FnOnce() -> Result<T, E>
{
	wrapped_ts_result(f())
}

#[inline]
pub async fn wrapped_ts_result_async<T, E, RE, F>(future: F)
	-> WrappedTSResult<T, E, RE>
where
	F: Future<Output = Result<T, E>>
{
	wrapped_ts_result(future.await)
}

#[macro_export]
macro_rules! impl_display {
	($struct:ident) => {
		impl ::std::fmt::Display for $struct {
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
