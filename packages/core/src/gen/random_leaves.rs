use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub(crate) struct RandomLeaves {
	block_id: String,
	y: Option<Vec<Option<u16>>>
}
