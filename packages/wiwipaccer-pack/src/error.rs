use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[repr(transparent)]
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub(crate) ErrorInner);

#[derive(Debug, Error)]
pub(crate) enum ErrorInner {
	#[error(
		"dependencies not satisfied: {}",
		.0.iter()
			.map(|(id, req, v)| match v {
				Some(v) => { format!("{id} {req} ({v} available)") }
				None => { format!("{id} {req}") }
			})
			.collect::<Vec<_>>()
			.join(", ")
	)]
	DepsNotSatisfied(Vec<(crate::nom::PackID, crate::nom::VersionReq, Option<semver::Version>)>),

	#[error("expected meta file to be a file:\n{0}")]
	MetaFileIsNotFile(String),

	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error("expected pack path to be a dir:\n{0}")]
	PackDirIsNotDir(String),

	#[error("error parsing semver:\n{0}")]
	SemverParseError(#[from] semver::Error),

	#[error(transparent)]
	TextureError(#[from] wiwipaccer_textures::error::Error),

	#[error(transparent)]
	UtilError(#[from] wiwipaccer_util::error::Error)
}
