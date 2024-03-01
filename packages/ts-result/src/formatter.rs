use crate::NiceErrorMessage;
use ::std::fmt::{ Arguments, Write };

const INDENT: &str = "   ";

/// basic formatter thingie
///
/// Only thing it does at the moment (or, at the time of writing this comment)
/// is to keep track of indents, so it can indent lines and stuff, so child errors
/// can be indented more than the parent or something like that
pub struct Formatter {
	string: String,
	indent: u8
}

impl Formatter {
	#[inline]
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let string = String::new();
		let indent = 0;
		Self { string, indent }
	}

	pub fn as_str(&self) -> &str {
		&self.string
	}

	#[inline]
	pub fn into_string(self) -> String {
		self.string
	}

	#[inline]
	fn init_indent(&mut self) {
		if self.string.ends_with('\n') {
			for _ in 0..self.indent {
				self.string.push_str(INDENT);
			}
		}
	}

	/// This function inserts what's written while in the body, with one extra
	/// level of indent, assuming that the body won't have leading or trailing
	/// newlines, and adds nothing of its own. You can call [`Formatter::next_line`]
	/// while in this function, and nest calls to itself too.
	#[inline]
	pub fn with_indent<F>(&mut self, f: F)
	where
		F: FnOnce(&mut Self)
	{
		self.indent += 1;
		self.init_indent();

		f(self);

		self.indent -= 1;
	}

	#[inline]
	pub fn write_args(&mut self, args: Arguments<'_>) {
		self.init_indent();

		if let Some(s) = args.as_str() {
			self.string.push_str(s);
		} else {
			self.string.write_fmt(args).unwrap();
		}
	}

	/// # Correctness
	///
	/// the str provided must not any new lines, otherwise the output
	/// will be malformed
	#[inline]
	pub fn write_str(&mut self, s: &str) {
		self.init_indent();
		self.string.push_str(s);
	}

	/// # Correctness
	///
	/// the char provided should not be newline
	#[inline]
	pub fn write_char(&mut self, c: char) {
		self.init_indent();
		self.string.push(c);
	}

	#[inline]
	pub fn next_line(&mut self) {
		self.string.push('\n');
	}

	#[inline]
	pub fn undo_next_line(&mut self) {
		if let Some(last_char) = self.string.pop() {
			if last_char != '\n' {
				self.string.push(last_char);
			}
		}
	}

	#[inline]
	pub fn write_line_args(&mut self, args: Arguments<'_>) {
		self.init_indent();
		self.write_args(args);
		self.next_line();
	}

	#[inline]
	pub fn write_line(&mut self, line: &str) {
		self.init_indent();
		self.write_str(line);
		self.next_line();
	}

	#[inline]
	pub fn fmt_with_indent<N>(&mut self, n: N)
	where
		N: NiceErrorMessage
	{
		self.with_indent(|f| n.fmt(f));
	}

	#[inline]
	pub fn fmt<N>(&mut self, n: N)
	where
		N: NiceErrorMessage
	{
		n.fmt(self);
	}
}
