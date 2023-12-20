#![allow(unused)]

pub mod error;
mod generators;
mod manifest;
mod util;

pub type Map<K, V> = hashbrown::hash_map::HashMap<K, V>;
pub type Result<V> = std::result::Result<V, error::Error>;
