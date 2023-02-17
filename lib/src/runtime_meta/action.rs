use crate::error::Result;
pub enum Action {
	CopyFile {
		from: String,
		to: String
	},
	WriteBytes {
		data: Vec<u8>,
		path: String,
		/// vec of paths of files that caused the compilation of this entry,
		/// the "source" if you will
		depends_on: Vec<String>
	},
	Process {
		f: Box<dyn Fn() -> Result<Vec<Action>>>
	}
}

impl std::fmt::Debug for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Action::*;
		match self {
			CopyFile { from, to } => {
				write!(f, "CopyFile {{ from: {from:?}, to: {to:?}}}")?;
			}
			WriteBytes { data, path, depends_on } => {
				write!(f, "WriteBytes {{ data: {data:?}, path: {path:?}, depends_on: {depends_on:?} }}")?;
			}
			Process { f: _ } => {
				write!(f, "Process {{ f: <closure> }}")?;
			}
		}

		Ok(())
	}
}
