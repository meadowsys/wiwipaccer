use ::mc_versions::MCVersionRef;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct RandomLeaves {
	block_id: String,
	y: Option<Vec<Option<u16>>>
}

impl RandomLeaves {
	#[inline]
	pub fn is_available_for(&self, mc_version: MCVersionRef) -> bool {
		false
	}
}
