// #[repr(C)]
// pub struct PluginMetadata {
// 	pub name: &'static str,
// 	pub id: &'static str,
// 	pub version: &'static str,
// 	pub authors: &'static [&'static str],
// 	pub properties: &'static [PluginProperty]
// }

// #[repr(C)]
// pub struct PluginProperty {
// 	pub required: bool,
// 	pub key: &'static str,
// 	pub val_type: PluginPropertyType
// }

// #[repr(C)]
// pub enum PluginPropertyType {
// 	Int,
// 	Float,
// 	String,
// 	Array(&'static PluginPropertyType),
// 	Map(&'static [PluginProperty])
// }

pub const fn å() -> &'static str {
	"aaaaaaaaaaaaaaaaaa"
}
