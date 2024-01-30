use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub(crate) struct RandomCubeAll {
	block_id: String,
	y: Option<Vec<Option<u16>>>,
	mirror: Option<bool>
}
