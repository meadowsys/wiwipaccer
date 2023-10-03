use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub enum Datasource {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	name: Option<String>,
	version: Option<DatasourceVersion>,
	description: Option<String>
}

#[derive(Deserialize, Serialize)]
pub enum DatasourceVersion {
	String(String),
	Git
}
