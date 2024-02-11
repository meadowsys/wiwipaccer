use ::ron::error::{ Error, Position, SpannedError };
use ::ts_result::*;

pub enum Ron {
	Error(Error),
	SpannedError(SpannedError)
}

impl From<Error> for Ron {
	fn from(error: Error) -> Self {
		Self::Error(error)
	}
}

impl From<SpannedError> for Ron {
	fn from(error: SpannedError) -> Self {
		Self::SpannedError(error)
	}
}

impl NiceErrorMessage for Ron {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("error parsing ron:");

		use Ron::*;
		f.with_indent(|f| match self {
			Error(e) => { f.write_args(format_args!("{e}")) }
			SpannedError(e) => { f.write_args(format_args!("{e}")) }
		});
	}
}
