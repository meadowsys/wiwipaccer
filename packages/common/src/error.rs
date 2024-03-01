use ::ts_result::*;

#[derive(Debug)]
pub struct OptionIDError {
	pub(crate) pack_id: Option<Box<ComponentError>>,
	pub(crate) texture_id: Option<Box<ComponentError>>,
	pub(crate) option_id: Option<Box<ComponentError>>,
}

#[derive(Debug)]
pub(crate) struct ComponentError {
	pub(crate) component: String,
	pub(crate) invalid_chars: Vec<char>
}

impl OptionIDError {
	pub(crate) fn blank() -> Self {
		let pack_id = None;
		let texture_id = None;
		let option_id = None;
		Self { pack_id, texture_id, option_id }
	}

	pub(crate) fn contains_error(&self) -> bool {
		if self.pack_id.is_some() { return true }
		if self.texture_id.is_some() { return true }
		if self.option_id.is_some() { return true }
		false
	}
}

impl NiceErrorMessage for OptionIDError {
	fn fmt(&self, f: &mut Formatter) {
		if let Some(pack_id) = &self.pack_id {
			f.write_line("Pack ID is invalid");
			f.with_indent(|f| f.fmt(&**pack_id));
			f.next_line();
		}

		if let Some(texture_id) = &self.texture_id {
			f.write_line("Texture ID is invalid");
			f.with_indent(|f| f.fmt(&**texture_id));
			f.next_line();
		}

		if let Some(option_id) = &self.option_id {
			f.write_line("Option ID is invalid");
			f.with_indent(|f| f.fmt(&**option_id));
			f.next_line();
		}

		f.write_str("ID components are only allowed to contain loweralpha, numeric, and dash characters");
	}
}

impl NiceErrorMessage for ComponentError {
	fn fmt(&self, f: &mut Formatter) {
		let Self { component, invalid_chars } = self;

		f.write_line_args(format_args!("provided component: {component}"));
		f.write_line("invalid characters found:");
		f.with_indent(|f| {
			invalid_chars.iter().copied().for_each(|c| {
				f.write_str("- '");
				f.write_char(c);
				f.write_char('\'');
				f.next_line();
			});
			f.undo_next_line();
		});
	}
}

impl_display!(OptionIDError);
impl Error for OptionIDError {}

/// ID components are only allowed to contain loweralpha, numeric, and dash characters
#[inline]
pub(crate) fn invalid_chars(id_component: &str) -> Option<Vec<char>> {
	#[inline]
	fn char_is_valid(c: char) -> bool {
		matches!(c, 'a'..='z' | '0'..='9' | '-')
	}

	let iter = id_component.chars()
		.filter(|c| !char_is_valid(*c));

	// dedupe, but preserving order
	let (hint_lower, hint_upper) = iter.size_hint();
	let vec = Vec::with_capacity(hint_upper.unwrap_or(hint_lower));
	let vec = iter.fold(vec, |mut acc, curr| {
		if !acc.contains(&curr) { acc.push(curr) }
		acc
	});

	if vec.is_empty() { None } else { Some(vec) }
}

#[cfg(test)]
mod tests {
	#[test]
	fn error_message() {
		let expected = include_str!("../test/fixtures/error-message.txt");
		let error = OptionID::builder()
			.pack_id("l&t")
			.texture_id("aha      ùwú")
			.option_id("ra andom")
			.build()
			.unwrap_err()
			.to_error_message();
		assert_eq!(expected, &*error);
	}
}
