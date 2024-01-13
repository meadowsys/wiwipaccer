use crate::error::*;
use ::surrealdb::Surreal;
use ::surrealdb::engine::local::{ Db, SpeeDb };
use ::surrealdb::opt::Config;
use ::surrealdb::dbs::Capabilities;

pub struct Data {
	surreal: Surreal<Db>
}

impl Data {
	pub async fn new(path: &str) -> Result<Self> {
		let capabilities = Capabilities::all();
		let cfg = Config::default()
			.capabilities(capabilities);
		let surreal = Surreal::new::<SpeeDb>((path, cfg))
			.await
			.map_err(into_err)?;
		Ok(Self { surreal })
	}
}
