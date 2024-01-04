// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::error::*;
use ::async_trait::async_trait;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: meta_nom::Name,
		pack_id: meta_nom::PackID,
		description: meta_nom::OptionalDescription,
		version: meta_nom::OptionalVersion,
		dependencies: meta_nom::Dependencies
	}
}

pub struct Pack {
	name: nom::Name,
	dir: nom::Dir,
	pack_id: nom::PackID,
	description: nom::OptionalDescription,
	version: nom::OptionalVersion,
	dependencies: nom::Dependencies
}

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub PackID, inner: String);
		nominal!(pub OptionalDescription, inner: Option<String>);
		nominal!(pub OptionalVersion, inner: Option<String>);
		nominal!(pub VersionReq, inner: String);
		nominal!(pub Dependencies, inner: HashMap<PackID, VersionReq>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub Dir, inner: String);
		nominal!(pub PackID, inner: String);
		nominal!(pub BorrowedPackID, inner: ref <'h> &'h str);
		nominal!(pub OptionalDescription, inner: Option<String>);
		nominal!(pub UnwrappedDescription, inner: String);
		nominal!(pub OptionalVersion, inner: Option<semver::Version>);
		nominal!(pub VersionReq, inner: semver::VersionReq);
		nominal!(pub BorrowedVersionReq, inner: ref <'h> &'h semver::VersionReq);
		nominal!(pub Dependencies, inner: HashMap<PackID, VersionReq>);
	}
}

impl Pack {
	#[inline]
	pub fn name(&self) -> &nom::Name {
		&self.name
	}

	#[inline]
	pub fn dir(&self) -> &nom::Dir {
		&self.dir
	}

	#[inline]
	pub fn pack_id(&self) -> &nom::PackID {
		&self.pack_id
	}

	#[inline]
	pub fn optional_description(&self) -> &nom::OptionalDescription {
		&self.description
	}

	#[inline]
	pub fn unwrap_description(&self) -> nom::UnwrappedDescription {
		let description = self.description
			.clone()
			.into_inner()
			.unwrap_or_else(|| "no description provided".into());
		nom::UnwrappedDescription::new(description)
	}

	#[inline]
	pub fn optional_version(&self) -> &nom::OptionalVersion {
		&self.version
	}

	#[inline]
	pub fn dependencies(&self) -> &nom::Dependencies {
		&self.dependencies
	}

	#[inline]
	pub fn has_dependencies(&self) -> bool {
		!self.dependencies.ref_inner().is_empty()
	}
}

#[async_trait]
pub trait DependencyResolver {
	type Dependency: Dependency;
	async fn dependency(
		&self,
		pack_id: nom::BorrowedPackID,
		version_req: nom::BorrowedVersionReq
	) -> Result<Option<Self::Dependency>>;
}

#[async_trait]
pub trait Dependency {}

impl Pack {
	pub async fn new<R, D>(dir: nom::Dir, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		todo!()
	}
}
