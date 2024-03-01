#![allow(clippy::new_without_default)]

mod error;
pub mod mc_id;
pub mod option_id;

pub use self::error::OptionIDError;
pub use self::mc_id::MinecraftID;
pub use self::option_id::OptionID;
