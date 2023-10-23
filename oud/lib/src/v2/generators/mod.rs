use camino::Utf8PathBuf;
use std::error::Error;

trait GeneratorConstructor {
	fn create_generator(&self, assets_path: Utf8PathBuf) -> Box<dyn Generator>;
}

trait Generator {
	fn build_actions(&self) -> Result<Vec<Action>, Box<dyn Error>>;
}

pub enum Action {
	WriteBytes {
		bytes: Vec<u8>,
		path: Utf8PathBuf,
		src_files: Vec<String>
	}
}

pub mod copy_paste;
