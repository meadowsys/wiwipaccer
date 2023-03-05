#![allow(non_upper_case_globals)]
use super::DATASTORE;
use surrealdb::Session;
use std::collections::BTreeMap;

const NS: &str = "e";
const DB: &str = "h";
const strict: bool = false;

pub async fn get_recent_projects() {
	let txt = "select * from recents order by time desc";
	let sess = &Session::for_kv()
		.with_db(DB)
		.with_ns(NS);
	let vars = None;

	let res = DATASTORE.read()
		.await
		.as_ref()
		.unwrap()
		.execute(txt, sess, vars, strict)
		.await
		.unwrap();
	println!("{res:?}");
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
	let sess = &Session::for_kv()
		.with_db(DB)
		.with_ns(NS);
	let mut vars = BTreeMap::new();
	vars.insert("path".into(), project_path.into());
	let vars = Some(vars);

	let res = DATASTORE.read()
		.await
		.as_ref()
		.unwrap()
		.execute(txt, sess, vars, strict)
		.await
		.unwrap();
	println!("{res:?}");
}
