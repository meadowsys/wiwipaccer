#![allow(unused)]

mod generators;
mod manifest;
mod util;

pub type Map<K, V> = hashbrown::hash_map::HashMap<K, V>;
