use crate::error::Result;
pub enum Action {
	CopyFile {
		from: String,
		/// this is relative to the base of the built resource pack
		to: String
	},
	WriteBytes {
		data: Vec<u8>,
		/// is is relative to the base of the built resource pack
		path: String,
		/// vec of paths of files that caused the compilation of this entry,
		/// the "source" if you will
		src_files: Vec<String>
	}
}

impl std::fmt::Debug for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Action::*;
		match self {
			CopyFile { from, to } => {
				write!(f, "CopyFile {{ from: {from:?}, to: {to:?}}}")?;
			}
			WriteBytes { data, path, src_files } => {
				write!(f, "WriteBytes {{ data: {data:?}, path: {path:?}, src_files: {src_files:?} }}")?;
			}
		}

		Ok(())
	}
}
