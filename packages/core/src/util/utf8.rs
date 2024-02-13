use crate::error::utf8_err::Utf8Error;
use ::std::ffi::OsStr;
use ::std::str::from_utf8;

#[inline]
pub fn osstr_to_str(os_str: &OsStr) -> Result<&str, Utf8Error> {
	from_utf8(os_str.as_encoded_bytes()).map_err(Into::into)
}
