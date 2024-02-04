use crate::error::individual::*;
use ::ts_result::*;

pub enum MCVersionError {
	InGet { version: UnknownMCVersion },
	InGetRangeFrom {
		from: UnknownMCVersion,
		to: String
	},
	InGetRangeTo {
		from: String,
		to: UnknownMCVersion
	}
}

impl NiceErrorMessage for MCVersionError {
	fn fmt(&self, f: &mut Formatter) {
		use MCVersionError::*;
		match self {
			InGet { version } => {
				f.write_line("could not fetch minecraft version");
				f.fmt_with_indent(version);
			}
			InGetRangeFrom { from, to } => {
				f.write_line("could not fetch minecraft version");
				f.with_indent(|f| {
					f.write_args(format_args!("from {} to {to}", from.version));
					f.next_line();

					f.write_str("reason: ");
					f.fmt(from);
				});
			}
			InGetRangeTo { from, to } => {
				f.write_line("could not fetch minecraft version");
				f.with_indent(|f| {
					f.write_args(format_args!("from {from} to {}", to.version));
					f.next_line();

					f.write_str("reason: ");
					f.fmt(to);
				});
			}
		}
	}
}

impl MCVersionError {
	pub fn in_getting(version: &str) -> Self {
		let version = UnknownMCVersion { version: version.into() };
		Self::InGet { version }
	}

	pub fn in_getting_range_from(from: &str, to: &str) -> Self {
		let from = UnknownMCVersion { version: from.into() };
		let to = to.into();
		Self::InGetRangeFrom { from, to }
	}

	pub fn in_getting_range_to(from: &str, to: &str) -> Self {
		let from = from.into();
		let to = UnknownMCVersion { version: to.into() };
		Self::InGetRangeTo { from, to }
	}
}
