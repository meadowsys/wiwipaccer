use wiwipaccer_provider_api::prelude::*;

struct Plugin;
export!(Plugin);

impl Guest for Plugin {
	fn meta() -> PluginMeta {
		PluginMeta {
			ns: "wiwi".into(),
			id: "cube-all".into(),
			version: Some(cargo_pkg_version!())
			// authors: vec![
			// 	"Meadow Liu <meadowsys@kiwin.gay>".into(),
			// 	api::h()
			// ]
		}
	}

	fn run(input: RunCtx) -> Result<PackEntries, Error> {
		let assets = input.get_assets()?;

		let y = match input.get_manifest_key("y") {
			ManifestKey::Array(y) => {
				// todo this can be a convenience method upstream
				y.into_iter()
					.enumerate()
					.map(|(i, y)| {
						let ManifestSubItemResult::Int(y) = y else {
							return Err(Error::from_str(&format!("got wrong type for `y[{i}]`, expected int")))
						};
						Ok(y)
					})
					.collect::<Result<Vec<_>, _>>()?
			}
			ManifestKey::None => { vec![] }
			_ => { return Err(Error::from_str("got wrong type for `y`, expected none or array of ints")) }
		};

		// todo these match statements can all be convenience methods upstream
		let mirror = match input.get_manifest_key("mirror") {
			ManifestKey::Bool(mirror) => { mirror }
			ManifestKey::None => { false }
			_ => { return Err(Error::from_str("got wrong type for `mirror`, expected none or bool")) }
		};

		let _ = assets.into_iter().map(|asset| {
			let path = asset.path();
			let bytes = asset.read_all()?;
			Ok::<_, Error>(())
		});

		todo!("wip lol")
	}
}
