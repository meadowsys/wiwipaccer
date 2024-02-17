use super::common::WithPath;
use ::ts_result::*;

#[derive(Debug)]
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

#[derive(Debug)]
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
		WithPath::new(self, path)
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

#[derive(Debug)]
pub struct IsDir {
	error: WithPath<Metadata>
}

impl IsDir {
	pub fn into_inner(self) -> MetadataWithPath {
		self.error
	}
}

impl NiceErrorMessage for IsDir {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("error checking if path is a dir"));
		f.with_indent(|f| {
			f.fmt(&self.error);
		})
	}
}

#[derive(Debug)]
pub struct IsFile {
	error: WithPath<Metadata>
}

impl IsFile {
	pub fn into_inner(self) -> MetadataWithPath {
		self.error
	}
}

impl NiceErrorMessage for IsFile {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("error checking if path is a file"));
		f.with_indent(|f| {
			f.fmt(&self.error);
		})
	}
}

#[derive(Debug)]
pub enum Read {
	FileSystem {
		error: ::std::io::Error
	},
	Join {
		error: SpawnBlocking
	}
}

impl NiceErrorMessage for Read {
	fn fmt(&self, f: &mut Formatter) {
		todo!()
	}
}

#[derive(Debug)]
pub enum ReadToString {
	Read(Read),
	UTF8 {
		error: ::std::str::Utf8Error,
		bytes: Vec<u8>
	}
}

impl From<Read> for ReadToString {
	fn from(value: Read) -> Self {
		Self::Read(value)
	}
}

impl NiceErrorMessage for ReadToString {
	fn fmt(&self, f: &mut Formatter) {
		todo!()
	}
}

#[derive(Debug)]
pub struct ReadDir {
	error: ::std::io::Error
}

pub type ReadDirWithPath = WithPath<ReadDir>;

impl NiceErrorMessage for ReadDir {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line("error reading directory:");
		f.write_args(format_args!("{}", self.error));
	}
}

#[derive(Debug)]
pub struct ReadDirEntry {
	error: ::std::io::Error
}

pub type ReadDirEntryWithPath = WithPath<ReadDirEntry>;

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

pub fn read_fs(error: ::std::io::Error) -> Read {
	Read::FileSystem { error }
}

pub fn read_join(error: SpawnBlocking) -> Read {
	Read::Join { error }
}

pub fn read_to_string_utf8(error: ::std::str::Utf8Error, bytes: Vec<u8>) -> ReadToString {
	ReadToString::UTF8 { error, bytes }
}

pub fn read_dir(error: ::std::io::Error, path: String) -> WithPath<ReadDir> {
	let error = ReadDir { error };
	WithPath::new(error, path)
}

pub fn read_dir_entry(error: ::std::io::Error, path: String) -> WithPath<ReadDirEntry> {
	let error = ReadDirEntry { error };
	WithPath::new(error, path)
}
