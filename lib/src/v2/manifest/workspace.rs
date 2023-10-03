// FIXME: this isn't finalised, need to think about it more

use camino::Utf8PathBuf;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub enum Workspace {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	name: String,
	paths: Vec<Utf8PathBuf>
}
