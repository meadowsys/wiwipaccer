use ::std::ops::{ Deref, FromResidual, Try };
use ::std::ops::ControlFlow::{ self, Break, Continue };
use ::ts_result::*;
use super::*;

pub enum CheckError {
	NotFile {
		path: String,
		getting: &'static str
	},
	NotDir {
		path: String,
		getting: &'static str
	},
	Metadata {
		error: fs_err::MetadataWithPath
	}
}

impl From<fs_err::IsDir> for CheckError {
	#[inline]
	fn from(value: fs_err::IsDir) -> Self {
		let error = value.into_inner();
		Self::Metadata { error }
	}
}

impl From<fs_err::IsFile> for CheckError {
	#[inline]
	fn from(value: fs_err::IsFile) -> Self {
		let error = value.into_inner();
		Self::Metadata { error }
	}
}

pub enum SilentResult {
	Ok(String),
	SilentFail,
	Err(CheckError)
}

pub enum SilentResultResidual {
	SilentFail,
	Err(CheckError)
}

impl Try for SilentResult {
	type Output = String;
	type Residual = SilentResultResidual;

	#[inline]
	fn from_output(output: Self::Output) -> Self {
		Self::Ok(output)
	}

	#[inline]
	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
		use SilentResult::*;
		match self {
			Ok(p) => { Continue(p) }
			SilentFail => { Break(SilentResultResidual::SilentFail) }
			Err(e) => { Break(SilentResultResidual::Err(e)) }
		}
	}
}

impl FromResidual for SilentResult {
	#[inline]
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		use SilentResultResidual::*;
		match residual {
			SilentFail => { SilentResult::SilentFail }
			Err(e) => { SilentResult::Err(e) }
		}
	}
}

impl<T, E> FromResidual<SilentResultResidual> for Result<Option<T>, E>
where
	CheckError: Into<E>
{
	#[inline]
	fn from_residual(residual: SilentResultResidual) -> Self {
		use SilentResultResidual::*;
		match residual {
			SilentFail => { Result::Ok(None) }
			Err(e) => { Result::Err(e.into()) }
		}
	}
}

pub fn not_file(path: String, getting: &'static str) -> CheckError {
	CheckError::NotFile { path, getting }
}

pub fn not_dir(path: String, getting: &'static str) -> CheckError {
	CheckError::NotDir { path, getting }
}
