// TODO: remove when more complete
#![allow(unused)]
#![deny(unused_must_use)]
#![allow(
	clippy::new_without_default
)]
#![deny(
	clippy::unwrap_in_result,
	clippy::unwrap_used
)]

pub mod error;
pub mod ron;
mod util;

pub mod pack_sources;
pub mod texture;
pub mod workspace;
