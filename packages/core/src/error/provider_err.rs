use super::{ path_builder_err, fs_err, ron_err };
use ::ts_result::*;

#[derive(Debug)]
pub struct DeserialiseMeta {
	error: ron_err::Ron
}

impl From<ron_err::Ron> for DeserialiseMeta {
	fn from(error: ron_err::Ron) -> Self {
		Self { error }
	}
}

impl NiceErrorMessage for DeserialiseMeta {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("error in deserialising meta for provider");
		f.with_indent(|f| f.fmt(&self.error))
	}
}

#[derive(Debug)]
pub enum New {
	CheckPath(path_builder_err::CheckError),
	DeserialiseMeta(DeserialiseMeta),
	ReadingMetaFile(fs_err::ReadToString)
}

impl From<path_builder_err::CheckError> for New {
	fn from(error: path_builder_err::CheckError) -> Self {
		Self::CheckPath(error)
	}
}

impl From<DeserialiseMeta> for New {
	fn from(error: DeserialiseMeta) -> Self {
		Self::DeserialiseMeta(error)
	}
}

pub(crate) fn reading_meta_file(error: fs_err::ReadToString) -> New {
	New::ReadingMetaFile(error)
}
