//! custom error type to avoid promise rejecting, and present an API similar to
//! [`Result`] in rust, by representing it as a union type

use serde::{ Serialize, Serializer };
use serde::ser::SerializeStruct;

pub enum TSResult<T: Serialize, E: NiceErrorMessage> {
	Ok(T),
	Err(E)
}

pub trait NiceErrorMessage {
	fn to_error_message(&self) -> String;
}

impl<T, E> Serialize for TSResult<T, E>
where
	T: Serialize,
	E: NiceErrorMessage
{
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		const SUCCESS: &str = "success";
		const VALUE: &str = "value";
		const ERROR: &str = "error";

		let mut s = s.serialize_struct("TSResult", 2)?;

		match self {
			TSResult::Ok(v) => {
				s.serialize_field(SUCCESS, &true)?;
				s.serialize_field(VALUE, v)?
			}
			TSResult::Err(e) => {
				s.serialize_field(SUCCESS, &false)?;
				s.serialize_field(ERROR, &e.to_error_message())?;
			}
		}

		s.end()
	}
}
