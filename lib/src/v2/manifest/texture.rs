use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub enum Texture {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	name: Option<String>,
	description: Option<String>,
	default: Option<String>
}
