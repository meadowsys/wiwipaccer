// FIXME: this isn't finalised, need to think about it more

use super::*;

#[derive(Deserialize, Serialize)]
pub enum Workspace {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	pub name: String,
	pub paths: Vec<Utf8PathBuf>
}
