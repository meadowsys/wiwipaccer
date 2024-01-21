use super::error::*;
use super::nr;

pub struct TextureRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	default: nr::Default,
	options: nr::Options
}

impl TextureRuntime {
	pub(crate) async fn new() -> Result<Self> {
		todo!()
	}
}
