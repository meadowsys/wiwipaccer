use ::ts_result::*;

#[derive(Debug)]
pub struct WithPath<E> {
	error: E,
	path: String
}

impl<E> WithPath<E> {
	pub fn new(error: E, path: String) -> Self {
		Self { error, path }
	}
}

impl<E: NiceErrorMessage> NiceErrorMessage for WithPath<E> {
	fn fmt(&self, f: &mut Formatter) {
		f.write_str("for path: ");
		f.write_str(&self.path);
		f.next_line();

		f.with_indent(|f| {
			f.fmt(&self.error);
		});
	}
}
