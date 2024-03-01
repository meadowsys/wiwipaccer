use ::ts_result::*;

#[derive(Debug)]
pub(crate) struct ComponentError {
	pub(crate) component: String,
	pub(crate) invalid_chars: Vec<char>
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

#[derive(Debug)]
pub struct OptionIDError {
	pub(crate) pack_id: Option<Box<ComponentError>>,
	pub(crate) texture_id: Option<Box<ComponentError>>,
	pub(crate) option_id: Option<Box<ComponentError>>,
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
		debug_assert!(self.contains_error());

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

impl_display!(OptionIDError);
impl Error for OptionIDError {}

#[derive(Debug)]
pub struct MinecraftIDError {
	pub(crate) ns: Option<Box<ComponentError>>,
	pub(crate) id: Option<Box<ComponentError>>
}

impl MinecraftIDError {
	pub(crate) fn blank() -> Self {
		let ns = None;
		let id = None;
		Self { ns, id }
	}

	pub(crate) fn contains_error(&self) -> bool {
		if self.ns.is_some() { return true }
		if self.id.is_some() { return true }
		false
	}
}

impl NiceErrorMessage for MinecraftIDError {
	fn fmt(&self, f: &mut Formatter) {
		debug_assert!(self.contains_error());

		if let Some(ns) = &self.ns {
			f.write_line("Namespace is invalid");
			f.with_indent(|f| f.fmt(&**ns));
			f.next_line();
		}

		if let Some(id) = &self.id {
			f.write_line("ID is invalid");
			f.with_indent(|f| f.fmt(&**id));
			f.next_line();
		}

		f.write_str("ID components are only allowed to contain loweralpha, numeric, and underscore characters");
	}
}

impl_display!(MinecraftIDError);
impl Error for MinecraftIDError {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{ MinecraftID, OptionID };

	#[test]
	fn option_id_error_message() {
		let expected = include_str!("../test/fixtures/option-id-error-message.txt");
		let error = OptionID::builder()
			.pack_id("l&t")
			.texture_id("aha      ùwú")
			.option_id("ra andom")
			.build()
			.unwrap_err()
			.to_error_message();
		assert_eq!(expected, &*error);
	}

	#[test]
	fn mc_id_error_message() {
		let expected = include_str!("../test/fixtures/minecraft-id-error-message.txt");
		let error = MinecraftID::builder()
			.namespace("mine craft")
			.id("stone héhé")
			.build()
			.unwrap_err()
			.to_error_message();
		assert_eq!(expected, &*error);
	}
}
