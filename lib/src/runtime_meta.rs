pub mod action;
pub mod version;

const META_NAME: &str = "manifest.wpm";
const ASSETS_DIR_NAME: &str = "assets";

#[derive(Debug)]
pub struct Warning {
	message: String
}
