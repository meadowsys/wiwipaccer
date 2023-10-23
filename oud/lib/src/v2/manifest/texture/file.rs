use super::*;

#[derive(Deserialize, Serialize)]
pub enum Texture {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	pub name: Option<String>,
	pub description: Option<String>,
	pub default: Option<String>
}
