use super::*;

#[derive(Deserialize, Serialize)]
pub enum Datasource {
	V1(V1)
}

#[derive(Deserialize, Serialize)]
pub struct V1 {
	pub name: Option<String>,
	pub version: Option<DatasourceVersion>,
	pub description: Option<String>
}

#[derive(Deserialize, Serialize)]
pub enum DatasourceVersion {
	String(String),
	Git
}
