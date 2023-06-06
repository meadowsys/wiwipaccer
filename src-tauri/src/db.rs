#![allow(non_upper_case_globals)]

use serde::{ Deserialize, Serialize };
use surrealdb::engine::local::RocksDb;
use surrealdb::engine::local::Db;
use surrealdb::opt::PatchOp;
use surrealdb::sql::{ Datetime, Id, Value };
use surrealdb::Surreal;
use std::collections::BTreeMap;
use tauri::async_runtime;
use tokio::sync::RwLock;

const NS: &str = "e";
const DB: &str = "h";
const RECENTS: &str = "recents";

lazy_static::lazy_static! {
	static ref DATASTORE: RwLock<Option<Surreal<Db>>> = RwLock::new(None);
}

pub fn init_db(datastore_path: &str) {
	let datastore = async_runtime::block_on(async { Surreal::new::<RocksDb>(datastore_path).await })
		.expect("Couldn't create datastore");
	async_runtime::block_on(async { datastore.use_ns(NS).use_db(DB).await })
		.expect("Couldn't set NS and DB to use");
	async_runtime::block_on(async {
		*DATASTORE.write().await = Some(datastore);
	});
}

pub fn drop_db() {
	let datastore = async_runtime::block_on(DATASTORE.write()).take().unwrap();
	drop(datastore);
	eprintln!("dropped datastore");
}

pub async fn get_recent_projects() -> Vec<String> {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let mut res = datastore.query("select * from recents order by time desc")
		.await
		.unwrap();

	let recents: Vec<Recent> = res.take(0).unwrap();

	recents.into_iter()
		.map(|r| r.path)
		.collect()
}

pub async fn add_recent_project(project_path: &str) {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let recent: Option<Recent> = datastore.select((RECENTS, project_path))
		.await
		.unwrap();

	let _: Recent = if let Some(recent) = recent {
		datastore.update((RECENTS, project_path))
			.content(Recent {
				path: recent.path,
				time: Datetime::default()
			})
			.await
			.unwrap()
	} else {
		datastore.create((RECENTS, project_path))
			.content(Recent {
				path: project_path.into(),
				time: Datetime::default()
			})
			.await
			.unwrap()
	};
}

pub async fn remove_recent_project(project_path: &str) {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let _: Recent = datastore.delete((RECENTS, project_path))
		.await
		.unwrap();
}

pub async fn clear_recent_projects() {
	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let _: Vec<Recent> = datastore.delete(RECENTS)
		.await
		.unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct Recent {
	path: String,
	time: Datetime
}
