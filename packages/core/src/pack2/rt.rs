use super::nr;

pub struct PackRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	dir: nr::Dir,
	version: nr::Version,
	dependencies: nr::Dependencies,
	textures: nr::Textures
}
