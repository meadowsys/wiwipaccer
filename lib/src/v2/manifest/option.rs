use super::*;

#[derive(Deserialize, Serialize)]
pub enum TextureOption {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	/// name of option
	name: Option<String>,
	/// description of option
	description: Option<String>
}
