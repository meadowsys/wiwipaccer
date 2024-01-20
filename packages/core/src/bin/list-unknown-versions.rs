use ::wiwipaccer_core::mc_versions::{ MC_VERSIONS, PackFormat::Unknown };

fn main() {
	println!("MC versions without a declared pack format:");

	MC_VERSIONS
		.iter()
		.filter(|v| matches!(v.pack_format, Unknown))
		.map(|v| v.name)
		.for_each(|v| println!("   {v}"));
}
