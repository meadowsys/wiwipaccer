use crate::texture2::{ self, TextureRuntime };
use crate::util::{ create_path_builder3, fs };
use crate::util::path_builder3::WithRootDir;
use super::error::*;
use super::{ meta, nm, nr };
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
		let p = create_path_builder3()
			.with_root_dir(dir);
		let dir = p.root_dir_checked().await?;
		let meta_path = p.root_manifest_checked().await?;

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
		let version = process_version(version)?;
		let dependencies = process_deps(dependencies, &dep_resolver).await?;

		// TODO: do something with dependencies in hashmap when actual logic is in
		// I think we'll need to pass to read_textures to process it
		let dependencies = dependencies.into_iter()
			.map(|(id, (_, req))| (id, req))
			.collect();
		let dependencies = nr::Dependencies::new(dependencies);

		let textures = read_textures(&p).await?;

		Ok(Self { name, description, id, dir, version, dependencies, textures })
	}
}

#[inline]
fn process_version(version: nm::Version) -> Result<nr::Version> {
	version
		.map_nom_some::<Dummy, _, _>(|v| ::semver::Version::parse(&v))
		.map_nom(Option::transpose)
		.transpose()
		.map_err(Into::into)
}

#[inline]
async fn process_deps<R, D>(deps: nm::Dependencies, dep_resolver: &R)
	-> Result<HashMap<nr::ID, (D, ::semver::VersionReq)>>
where
	R: DependencyResolver<Dependency = D>
{
	let deps = deps
		.into_inner()
		.unwrap_or_default();

	let mut satisfied = HashMap::with_capacity(deps.len());
	let mut not_satisfied = Vec::with_capacity(deps.len());

	for (id, req) in deps {
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

	Ok(satisfied)
}

#[inline]
async fn read_textures(p: &WithRootDir<'_>) -> Result<nr::Textures> {
	let textures_dir = p.texture_entries_dir_checked().await?;
	let mut textures_nom = nr::Textures::default();
	let textures = textures_nom.mut_inner();

	let mut read_dir = fs::read_dir2(textures_dir).await?;

	while let Some(file) = read_dir.next().await? {
		let file_name = file.file_name();
		let p = p.clone().with_texture_id_osstr(&file_name)?;

		// TODO
		if let Some(t) = TextureRuntime::new(&p).await? {
			let id = texture2::nr::ID::new(p.texture_id_ref().into());
			textures.insert(id, t);
		}
	}

	Ok(textures_nom)
}
