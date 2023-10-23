//! blockstate struct
//!
//! NOWHERE NEAR FINISHED h

use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use ahash::RandomState;

#[derive(Debug, Deserialize, Serialize)]
pub struct Blockstate {
	pub variants: HashMap<String, Vec<BlockstateEntry>, RandomState>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockstateEntry {
	pub model: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub y: Option<u16>
}
