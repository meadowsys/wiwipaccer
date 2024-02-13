use ::std::str;
use ::ts_result::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct Utf8Error {
	error: str::Utf8Error
}

impl From<str::Utf8Error> for Utf8Error {
	fn from(error: str::Utf8Error) -> Self {
		Utf8Error { error }
	}
}

impl NiceErrorMessage for Utf8Error {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("invalid utf8");
		f.with_indent(|f| f.write_args(format_args!("{}", self.error)));
	}
}
