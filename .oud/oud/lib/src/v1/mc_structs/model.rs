//! model struct
//!
//! NOWHERE NEAR FINISHED h

use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use ahash::RandomState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
	pub parent: String,
	pub textures: Option<HashMap<String, String, RandomState>>
}
