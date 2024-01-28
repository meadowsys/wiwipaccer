use super::error::*;
use super::nr;

pub struct OptionRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	versions: nr::Versions
}

impl OptionRuntime {
	pub(crate) async fn new() -> Result<Option<Self>> {
		todo!()
	}
}
