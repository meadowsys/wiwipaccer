use ::hashbrown::HashMap;

::nominal::nominal_mod! {
	pub mod global {
		nominal!(pub Path, inner: String);
		nominal!(pub DirPath, inner: String);
		nominal!(pub FilePath, inner: String);
		nominal!(pub RootDirPath, inner: String);
	}

	pub mod workspace {
		nominal!(pub Name, inner: String);
		nominal!(pub Packs, inner: HashMap<pack::ID, crate::pack::Pack>);
		nominal!(pub PackIDs, inner: Vec<pack::ID>);
	}

	pub mod workspace_m {
		nominal!(pub Packs, inner: Vec<global::Path>);
	}

	pub mod pack {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
		nominal!(pub Version, inner: Option<semver::Version>);
		nominal!(pub Dependencies, inner: HashMap<ID, semver::VersionReq>);
		nominal!(pub Textures, inner: HashMap<texture::ID, crate::texture::Texture>);

		nominal!(pub DescriptionUnwrapped, inner: String);
	}

	pub mod pack_m {
		nominal!(pub Version, inner: Option<String>);
		nominal!(pub VersionReq, inner: String);
		nominal!(pub Dependencies, inner: Option<HashMap<pack::ID, VersionReq>>);
	}

	pub mod texture {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
	}

	pub mod texture_m {}

	pub mod option {
		nominal!(pub Name, inner: String);
		nominal!(pub Description, inner: Option<String>);
		nominal!(pub ID, inner: String);
	}

	pub mod option_m {}

	pub mod version {
		// nominal!(pub ID, inner: String);
	}

	pub mod version_m {}
}
