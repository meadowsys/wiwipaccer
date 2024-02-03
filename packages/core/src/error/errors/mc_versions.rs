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
	fn to_error_message(&self) -> String {
		use MCVersionError::*;
		match self {
			InGet { version } => {
				format!("could not fetch minecraft version: {version}")
			}
			InGetRangeFrom { from, to } => {
				format!(
					"could not fetch start minecraft version in range from `{from}` to `{to}`: {error}",
					from = from.version,
					error = from
				)
			}
			InGetRangeTo { from, to } => {
				format!(
					"could not fetch start minecraft version in range from `{from}` to `{to}`: {error}",
					to = to.version,
					error = to
				)
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
