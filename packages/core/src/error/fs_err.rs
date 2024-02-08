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

pub enum MetadataWithoutPath {
	FileSystem {
		error: ::std::io::Error
	},
	JoinError {
		error: SpawnBlocking
	}
}

impl MetadataWithoutPath {
	pub fn with_path(self, path: String) -> Metadata {
		let error = self;
		Metadata { error, path }
	}
}

impl NiceErrorMessage for MetadataWithoutPath {
	fn fmt(&self, f: &mut Formatter) {
		f.write_args(format_args!("error fetching metadata"));

		use MetadataWithoutPath::*;
		f.with_indent(|f| {
			match self {
				FileSystem { error } => {
					f.write_args(format_args!("{error}"));
				}
				JoinError { error } => {
					f.fmt(error);
				}
			}
		});
	}
}

pub struct Metadata {
	error: MetadataWithoutPath,
	path: String
}


impl NiceErrorMessage for Metadata {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("for path {}", self.path));
		f.fmt(&self.error);
	}
}

pub struct IsDir {
	error: Metadata,
	path: String
}

impl NiceErrorMessage for IsDir {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("error checking if path is a dir"));
		f.with_indent(|f| {
			f.write_str("path: ");
			f.write_str(&self.path);
			f.next_line();
			f.fmt(&self.error);
		})
	}
}

pub struct IsFile {
	error: Metadata,
	path: String
}

impl NiceErrorMessage for IsFile {
	fn fmt(&self, f: &mut Formatter) {
		f.write_line_args(format_args!("error checking if path is a file"));
		f.with_indent(|f| {
			f.write_str("path: ");
			f.write_str(&self.path);
			f.next_line();
			f.fmt(&self.error);
		})
	}
}

// pub enum ReadToString {
// 	Read {
// 		error: ::std::io::Error
// 	},
// 	Utf8 {
// 		error: ::std::str::Utf8Error,
// 		bytes: Vec<u8>
// 	}
// }

// pub struct ReadDir {
// 	error: ::std::io::Error
// }



pub fn spawn_blocking(error: ::tokio::task::JoinError) -> SpawnBlocking {
	SpawnBlocking { error }
}

pub fn metadata_fs<F>(path_fn: F)
	-> impl FnOnce(::std::io::Error) -> Metadata
where
	F: FnOnce() -> String
{
	|error| Metadata {
		error: MetadataWithoutPath::FileSystem { error },
		path: path_fn()
	}
}

pub fn metadata_join<F>(path_fn: F)
	-> impl FnOnce(SpawnBlocking) -> Metadata
where
	F: FnOnce() -> String
{
	|error| Metadata {
		error: MetadataWithoutPath::JoinError { error },
		path: path_fn()
	}
}

pub fn metadata_without_path_fs(error: ::std::io::Error)
	-> MetadataWithoutPath
{
	MetadataWithoutPath::FileSystem { error }
}

pub fn metadata_without_path_join(error: SpawnBlocking)
	-> MetadataWithoutPath
{
	MetadataWithoutPath::JoinError { error }
}

pub fn is_dir(error: Metadata, path: String) -> IsDir {
	IsDir { error, path }
}

pub fn is_file(error: Metadata, path: String) -> IsFile {
	IsFile { error, path }
}
