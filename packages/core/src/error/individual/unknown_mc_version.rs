use ::ts_result::*;

pub struct UnknownMCVersion {
	pub version: String
}

impl NiceErrorMessage for UnknownMCVersion {
	fn fmt(&self, f: &mut Formatter) {
		f.write_args(format_args!("unknown minecraft version {}", self.version));
	}
}
