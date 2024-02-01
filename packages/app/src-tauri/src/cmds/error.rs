//! custom error type to avoid promise rejecting, present an API similar to
//! Result in rust, by representing it as a union type

use serde::{ Serialize, Serializer };

#[derive(Serialize)]
pub enum NonThrowingTSResult<T: Serialize, E: GoodErrorMessage> {
	Ok(T),
	Err(E)
}

pub trait GoodErrorMessage {
	fn to_error_message(&self) -> String;
}

impl Serialize for &dyn GoodErrorMessage {
	#[inline]
	fn serialize<S>(&self, serialiser: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		serialiser.serialize_str(&self.to_error_message())
	}
}
