use ::ts_result::*;

struct UnknownMCVersion {
	version: String
}

impl NiceErrorMessage for UnknownMCVersion {
	fn fmt(&self, f: &mut Formatter) {
		f.write_args(format_args!("unknown minecraft version {}", self.version));
	}
}

pub struct InGetting {
	version: UnknownMCVersion
}

impl NiceErrorMessage for InGetting {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("could not fetch minecraft version");
		f.fmt_with_indent(&self.version);
	}
}

pub struct InGettingRangeFrom {
	from: UnknownMCVersion,
	to: String
}

impl NiceErrorMessage for InGettingRangeFrom {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("could not fetch minecraft version");
		f.with_indent(|f| {
			f.write_args(format_args!("from {} to {}", self.from.version, self.to));
			f.next_line();

			f.write_str("reason: ");
			f.fmt(&self.from);
		});
	}
}

pub struct InGettingRangeTo {
	from: String,
	to: UnknownMCVersion
}

impl NiceErrorMessage for InGettingRangeTo {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("could not fetch minecraft version");
		f.with_indent(|f| {
			f.write_args(format_args!("from {} to {}", self.from, self.to.version));
			f.next_line();

			f.write_str("reason: ");
			f.fmt(&self.to);
		});
	}
}

pub enum InGettingRange {
	From(InGettingRangeFrom),
	To(InGettingRangeTo)
}

impl NiceErrorMessage for InGettingRange {
	fn fmt(&self, f: &mut Formatter) {
		match self {
			Self::From(from) => { f.fmt(from) }
			Self::To(to) => { f.fmt(to) }
		}
	}
}

pub fn in_getting(version: &str) -> InGetting {
	let version = UnknownMCVersion { version: version.into() };
	InGetting { version }
}

pub fn in_getting_range_from(from: &str, to: &str) -> InGettingRange {
	let from = UnknownMCVersion { version: from.into() };
	let to = to.into();
	InGettingRange::From(InGettingRangeFrom { from, to })
}

pub fn in_getting_range_to(from: &str, to: &str) -> InGettingRange {
	let from = from.into();
	let to = UnknownMCVersion { version: to.into() };
	InGettingRange::To(InGettingRangeTo { from, to })
}
