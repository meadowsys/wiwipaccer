// TODO: remove when more complete
#![allow(unused)]
#![deny(unused_must_use)]
#![deny(
	clippy::unwrap_in_result,
	clippy::unwrap_used
)]

pub mod error;
pub mod ron;
mod util;
pub mod nominal_typing_owo;

pub mod pack_sources;
pub mod texture;
pub mod workspace;
