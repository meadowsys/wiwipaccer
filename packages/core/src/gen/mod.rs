mod random_cube_all;
mod random_leaves;

pub use self::random_cube_all::RandomCubeAll;
pub use self::random_leaves::RandomLeaves;
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

// #[derive(Deserialize, Serialize)]
// pub(super) enum PackVersionSpecMeta {
// 	PackVersion(u8),
// 	MCVersion(String),
// 	MCVersionRange(String, String)
// }
