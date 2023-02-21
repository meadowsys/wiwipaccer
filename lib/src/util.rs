use crate::error::{ Error, Result };
use ron::extensions::Extensions;
use tokio::fs;

lazy_static::lazy_static! {
	pub static ref RON: ron::Options = {
		ron::Options::default()
			.with_default_extension(Extensions::IMPLICIT_SOME)
			.with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
	};
}

#[async_recursion::async_recursion]
pub async fn walk_dir(path: &str) -> Result<Vec<String>> {
	let metadata = fs::metadata(path).await
		.map_err(|e| Error::FileDoesNotExist { path: path.into(), source: e })?;
	if !metadata.is_dir() { return Ok(vec![path.into()]) }

	let mut paths = vec![];
	let mut inner_files = fs::read_dir(path).await
		.map_err(|e| Error::IOError { source: e })?;

	while let Some(file) = inner_files.next_entry().await.map_err(|e| Error::IOError { source: e })? {
		let file = file.path();
		for file in walk_dir(file.to_str().expect("invalid unicode paths unsupported")).await? {
			paths.push(file);
		}
	}

	Ok(paths)
}

pub fn hash(thing: &str) -> String {
	// windows gets shorter paths (we don't want windows users to have to suffer,
	// smh my head windows with small path limit)
	// everyone else gets less likely to collide paths
	// realistically? 10 is enough to prevent collission 99.999999999% of the time,
	// but I like theoretical numbers go brrrrrr
	// actually took a while to convince myself that blake3 output, while half the
	// length of sha-512 (and sha3-512), is perfectly adequate and way secure lol
	//
	// but anyways
	// h

	// also this would mean that theoretically mac/linux users would have a
	// higher memory use, but unless you have a crazy big pack, its probably not
	// going to go over +5-10MB, generously estimating. Like, 32 bytes to 10,
	// extra 22 bytes per hash, 4,545,454 ish hashes would be needed to achieve +10MB
	// compared to windows. Besides, windows' high memory usage more than makes up
	// for this, lellelel

	// lol I do a ramble (I'm leaving this here h)

	// you're cute

	let hex = blake3::hash(thing.as_bytes()).to_hex();

	#[cfg(target_os = "windows")]
	let rv = hex[0..10].to_string();
	#[cfg(not(target_os = "windows"))]
	let rv = hex.to_string();

	rv
}
