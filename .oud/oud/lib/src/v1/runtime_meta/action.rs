use crate::error::{ Error, Result };
use std::path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub enum Action {
	CopyFile {
		/// this must be absolute
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

impl Action {
	pub async fn execute_in(&self, base_dir: &str) -> Result<()> {
		use Action::*;
		match self {
			CopyFile { from, to } => {
				let mut to_path = path::PathBuf::new();
				to_path.push(base_dir);
				to_path.push(to);

				if fs::metadata(&to_path).await.is_ok() {
					return Err(Error::FileAlreadyExists { path: to_path.to_str().unwrap().into() })
				}

				fs::create_dir_all(to_path.parent().unwrap()).await
					.map_err(|e| Error::IOError { source: e })?;

				fs::copy(from, to_path).await
					.map_err(|e| Error::IOError { source: e })?;
			}
			WriteBytes { data, path, src_files: _ } => {
				let mut to_path = path::PathBuf::new();
				to_path.push(base_dir);
				to_path.push(path);

				fs::create_dir_all(to_path.parent().unwrap()).await
					.map_err(|e| Error::IOError { source: e })?;

				let mut file = fs::OpenOptions::new()
					.create_new(true)
					.write(true)
					.open(&to_path)
					.await
					.map_err(|e| Error::IOError { source: e })?;

				file.write_all(data).await
					.map_err(|e| Error::IOError { source: e })?;
			}
		}

		Ok(())
	}
}
