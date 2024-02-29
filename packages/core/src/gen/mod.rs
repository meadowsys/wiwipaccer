#![deprecated]

mod random_cube_all;
mod random_leaves;

pub use self::random_cube_all::RandomCubeAll;
pub use self::random_leaves::RandomLeaves;
use ::mc_versions::MCVersionRef;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Generator {
	#[serde(rename = "random-cube-all")]
	RandomCubeAll {
		#[serde(flatten)]
		gen: RandomCubeAll
	},
	#[serde(rename = "random-leaves")]
	RandomLeaves {
		#[serde(flatten)]
		gen: RandomLeaves
	}
}

impl Generator {
	pub fn is_available_for(&self, mc_version: MCVersionRef) -> bool {
		use Generator::*;
		match self {
			RandomCubeAll { gen } => { gen.is_available_for(mc_version) }
			RandomLeaves { gen } => { gen.is_available_for(mc_version) }
		}
	}
}

// #[derive(Deserialize, Serialize)]
// pub(super) enum PackVersionSpecMeta {
// 	PackVersion(u8),
// 	MCVersion(String),
// 	MCVersionRange(String, String)
// }
