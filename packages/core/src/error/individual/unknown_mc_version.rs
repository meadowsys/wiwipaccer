use ::ts_result::*;

pub struct UnknownMCVersion {
	pub version: String
}

impl NiceErrorMessage for UnknownMCVersion {
	fn to_error_message(&self) -> String {
		let Self { version } = self;
		format!("unknown minecraft version {version}")
	}
}

impl_display!(UnknownMCVersion);
