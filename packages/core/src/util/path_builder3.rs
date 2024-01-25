use super::error::*;
use super::fs;
use ::camino::Utf8PathBuf;
use ::std::ops::Deref;

// -- structs --

// lol, not really necessary at all, but eh lel
pub struct Blank {
	__private: ()
}

#[derive(Clone, Debug)]
pub struct WithRootDir<'h> {
	root_dir: &'h str
}

#[derive(Clone, Debug)]
pub struct WithTextureID<'h> {
	with_root_dir: WithRootDir<'h>,
	texture_id: &'h str
}

#[derive(Clone, Debug)]
pub struct WithOptionID<'h> {
	with_texture_id: WithTextureID<'h>,
	option_id: &'h str
}

#[derive(Clone, Debug)]
pub struct WithVersionID<'h> {
	with_option_id: WithOptionID<'h>,
	version_id: &'h str
}

// -- creation / upgrade fns --

pub fn create_path_builder3() -> Blank {
	Blank { __private: () }
}

impl Blank {
	pub fn with_root_dir(self, root_dir: &str) -> WithRootDir {
		WithRootDir { root_dir }
	}
}

impl<'h> WithRootDir<'h> {
	pub fn with_texture_id(self, texture_id: &'h str) -> WithTextureID {
		WithTextureID { with_root_dir: self, texture_id }
	}
}

impl<'h> WithTextureID<'h> {
	pub fn with_option_id(self, option_id: &'h str) -> WithOptionID {
		WithOptionID { with_texture_id: self, option_id }
	}
}

impl<'h> WithOptionID<'h> {
	pub fn with_version_id(self, version_id: &'h str) -> WithVersionID {
		WithVersionID { with_option_id: self, version_id }
	}
}

// -- deref impls --

impl<'h> Deref for WithTextureID<'h> {
	type Target = WithRootDir<'h>;
	fn deref(&self) -> &Self::Target {
		&self.with_root_dir
	}
}

impl<'h> Deref for WithOptionID<'h> {
	type Target = WithTextureID<'h>;
	fn deref(&self) -> &Self::Target {
		&self.with_texture_id
	}
}

impl<'h> Deref for WithVersionID<'h> {
	type Target = WithOptionID<'h>;
	fn deref(&self) -> &Self::Target {
		&self.with_option_id
	}
}

// -- logic impls (private fns) --

// -- public interface --
