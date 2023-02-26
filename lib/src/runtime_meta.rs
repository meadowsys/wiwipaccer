pub mod action;
pub mod datasource;
pub mod option;
pub mod pack_version_specifier;
pub mod texture;
pub mod version;

const META_NAME: &str = "manifest.wpm";
const ASSETS_DIR_NAME: &str = "assets";
const TEXTURES_DIR: &str = "textures";

#[derive(Debug)]
pub struct Message {
	pub message: String,
	pub severity: MessageSeverity
}

#[derive(Debug)]
pub enum MessageSeverity {
	Info,
	Warning,
	Error,
	Fatal
}
