use crate::texture2;
use crate::util::{ fs, path_builder2 };
use super::error::*;
use super::{ meta, nr };
use ::async_trait::async_trait;
use ::hashbrown::HashMap;
use ::nominal::Dummy;

pub struct PackRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	dir: nr::Dir,
	version: nr::Version,
	dependencies: nr::Dependencies,
	textures: nr::Textures
}

#[async_trait]
pub trait DependencyResolver {
	type Dependency: Dependency;
	async fn dependency(
		&self,
		id: &nr::ID,
		version_req: &::semver::VersionReq
	) -> Result<DependencyResult<Self::Dependency>>;
}

#[async_trait]
pub trait Dependency {}

pub enum DependencyResult<D> {
	/// Found and satisfies version requirement
	Found(D),

	/// Not available
	NotFound,

	/// Found, but doesn't satisfy the version requirement.
	///
	/// The version is included in this enum variant so
	/// it can be included in an error message
	VersionNotSatisfied(::semver::Version)
}

impl PackRuntime {
	pub(crate) async fn new<R, D>(dir: &str, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		let p = path_builder2(dir);
		let dir = p.root_dir2().await?;
		let meta_path = p.root_manifest2().await?;

		let meta_file = fs::read_to_string2(meta_path).await?;
		let meta::PackUnversioned {
			name,
			description,
			id,
			version,
			dependencies
		} = meta::deserialise_pack(&meta_file)?;

		let name = name.transmute_nom();
		let description = description.transmute_nom();
		let id = id.transmute_nom();
		let dir = nr::Dir::new(dir);
		let version = version
			.map_nom_some::<Dummy, _, _>(|v| ::semver::Version::parse(&v))
			.map_nom(Option::transpose)
			.transpose()?;

		let dependencies = {
			let dependencies = dependencies
				.into_inner()
				.unwrap_or_default();
			let mut satisfied = HashMap::with_capacity(dependencies.len());
			let mut not_satisfied = Vec::with_capacity(dependencies.len());

			for (id, req) in dependencies {
				let id = nr::ID::new(id.into_inner());
				let req = ::semver::VersionReq::parse(req.ref_inner())?;

				let dep = {
					use DependencyResult::*;
					match dep_resolver.dependency(&id, &req).await? {
						Found(d) => { d }
						NotFound => {
							not_satisfied.push((id, req, None));
							continue
						}
						VersionNotSatisfied(v) => {
							not_satisfied.push((id, req, Some(v)));
							continue
						}
					}
				};

				satisfied.insert(id, (dep, req));
			}

			if !not_satisfied.is_empty() {
				return Err(Error::DepsNotSatisfied(not_satisfied))
			}

			satisfied
		};

		// TODO: do something with dependencies in hashmap when actual logic is in

		let dependencies = dependencies.into_iter()
			.map(|(id, (_, req))| (id, req))
			.collect();
		let dependencies = nr::Dependencies::new(dependencies);

		let textures = {
			let textures_dir = p.textures_path2().await?;
			let mut textures_nom = nr::Textures::default();
			let textures = textures_nom.mut_inner();

			let mut read_dir = fs::read_dir2(textures_dir.clone()).await?;

			while let Some(file) = read_dir.next().await? {
				let id = file.file_name();
				let id = id.to_str()
					.ok_or_else(|| Error::NonUtf8Path)?;
				let id = texture2::nr::ID::new(id.into());

				// TODO
				let texture = texture2::TextureRuntime::new().await?;

				if let Some(t) = texture {
					textures.insert(id, t);
				}
			}

			textures_nom
		};

		Ok(Self { name, description, id, dir, version, dependencies, textures })
	}
}
