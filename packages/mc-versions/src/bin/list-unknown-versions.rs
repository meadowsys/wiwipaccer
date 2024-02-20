use ::lazy_wrap::LazyWrap;
use ::mc_versions::{ MC_VERSIONS, PackFormat::Unknown };

fn main() {
	let printer = LazyWrap::new(|| {
		println!("MC versions without a declared pack format:");
		|v| println!("   {v}")
	});

	MC_VERSIONS
		.iter()
		.filter(|v| matches!(v.pack_format, Unknown))
		.map(|v| v.name)
		.for_each(|v| (*printer)(v));

	if !LazyWrap::is_initialised(&printer) {
		println!("everything declared!");
	}
}
