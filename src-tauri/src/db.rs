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
	let mut res = DATASTORE.read()
		.await
		.as_ref()
		.unwrap()
		.query("select * from recents order by time desc")
		.await
		.unwrap();

	let recents: Vec<Recent> = res.take(0).unwrap();
	dbg!(&recents);

	recents.into_iter()
		.map(|r| r.path)
		.collect()
}

pub async fn add_recent_project(project_path: &str) {
	let txt = r#"
		let $record_id = type::thing("recents", $path);
		let $existing = (select * from $record_id);

		if array::len($existing) = 0 then
			(create $record_id set path = $path, time = time::now())
		else
			(update $record_id set time = time::now())
		end
	"#;

	let datastore = DATASTORE.read()
		.await;
	let datastore = datastore.as_ref()
		.unwrap();

	let recent: Option<Recent> = datastore.select(("recents", project_path))
		.await
		.unwrap();
	dbg!(&recent);

	if let Some(recent) = recent {
		let e: Recent = datastore.update(("recents", project_path))
			.content(Recent {
				path: recent.path,
				time: Datetime::default()
			})
			.await
			.unwrap();
		dbg!(e);
	} else {
		let e: Recent = datastore.create(("recents", project_path))
			.content(Recent {
				path: project_path.into(),
				time: Datetime::default()
			})
			.await
			.unwrap();
		dbg!(e);
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct Recent {
	path: String,
	time: Datetime
}
