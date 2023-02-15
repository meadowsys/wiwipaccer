use serde::{ Deserialize, Serialize };
#[derive(Deserialize, Serialize)]
pub enum Datasource {
	DatasourceV1 {
		name: Option<String>,
		version: Option<Version>,
		description: Option<String>
	}
}

#[derive(Deserialize, Serialize)]
pub enum Version {
	String(String),
	Git
}
