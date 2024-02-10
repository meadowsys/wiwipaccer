use ::ts_result::*;

pub struct SpawnBlocking {
	error: ::tokio::task::JoinError
}

impl NiceErrorMessage for SpawnBlocking {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("tokio failed to run background task"));
		f.with_indent(|f| {
			f.write_args(format_args!("{}", self.error));
		});
	}
}

pub struct WithPath<E> {
	error: E,
	path: String
}

impl<E: NiceErrorMessage> NiceErrorMessage for WithPath<E> {
	fn fmt(&self, f: &mut Formatter) {
		f.write_str("for path: ");
		f.write_str(&self.path);
		f.next_line();

		f.with_indent(|f| {
			f.fmt(&self.error);
		});
	}
}

pub enum Metadata {
	FileSystem {
		error: ::std::io::Error
	},
	Join {
		error: SpawnBlocking
	}
}

pub type MetadataWithPath = WithPath<Metadata>;

impl Metadata {
	pub fn with_path(self, path: String) -> WithPath<Self> {
		WithPath { error: self, path }
	}
}

impl NiceErrorMessage for Metadata {
	fn fmt(&self, f: &mut Formatter) {
		f.write_args(format_args!("error fetching metadata"));

		use Metadata::*;
		f.with_indent(|f| match self {
			FileSystem { error } => {
				f.write_args(format_args!("{error}"));
			}
			Join { error } => {
				f.fmt(error);
			}
		});
	}
}

pub struct IsDir {
	error: WithPath<Metadata>
}

impl NiceErrorMessage for IsDir {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("error checking if path is a dir"));
		f.with_indent(|f| {
			f.fmt(&self.error);
		})
	}
}

pub struct IsFile {
	error: WithPath<Metadata>
}

impl NiceErrorMessage for IsFile {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("error checking if path is a file"));
		f.with_indent(|f| {
			f.fmt(&self.error);
		})
	}
}

pub enum ReadToString {
	FileSystem {
		error: ::std::io::Error
	},
	Join {
		error: SpawnBlocking
	},
	UTF8 {
		error: ::std::str::Utf8Error,
		bytes: Vec<u8>
	}
}

impl NiceErrorMessage for ReadToString {
	fn fmt(&self, f: &mut Formatter) {
		todo!()
	}
}

pub struct ReadDir {
	error: ::std::io::Error
}

impl NiceErrorMessage for ReadDir {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("error reading directory:");
		f.write_args(format_args!("{}", self.error));
	}
}

pub struct ReadDirEntry {
	error: ::std::io::Error
}

impl NiceErrorMessage for ReadDirEntry {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("error reading next dir entry:");
		f.write_args(format_args!("{}", self.error));
	}
}

pub fn spawn_blocking(error: ::tokio::task::JoinError) -> SpawnBlocking {
	SpawnBlocking { error }
}

pub fn metadata_fs(error: ::std::io::Error)
	-> Metadata
{
	Metadata::FileSystem { error }
}

pub fn metadata_join(error: SpawnBlocking)
	-> Metadata
{
	Metadata::Join { error }
}

pub fn is_dir(error: WithPath<Metadata>) -> IsDir {
	IsDir { error }
}

pub fn is_file(error: WithPath<Metadata>) -> IsFile {
	IsFile { error }
}

pub fn read_to_string_fs(error: ::std::io::Error) -> ReadToString {
	ReadToString::FileSystem { error }
}

pub fn read_to_string_join(error: SpawnBlocking) -> ReadToString {
	ReadToString::Join { error }
}

pub fn read_to_string_utf8(error: ::std::str::Utf8Error, bytes: Vec<u8>) -> ReadToString {
	ReadToString::UTF8 { error, bytes }
}

pub fn read_dir(error: ::std::io::Error, path: String) -> WithPath<ReadDir> {
	let error = ReadDir { error };
	WithPath { error, path }
}

pub fn read_dir_entry(error: ::std::io::Error, path: String) -> WithPath<ReadDirEntry> {
	let error = ReadDirEntry { error };
	WithPath { error, path }
}
