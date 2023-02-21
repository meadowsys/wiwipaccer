pub mod action;
pub mod datasource;
pub mod option;
pub mod texture;
pub mod version;

const META_NAME: &str = "manifest.wpm";
const ASSETS_DIR_NAME: &str = "assets";
const TEXTURES_DIR: &str = "textures";

#[derive(Debug)]
pub struct Warning {
	pub message: String
}
