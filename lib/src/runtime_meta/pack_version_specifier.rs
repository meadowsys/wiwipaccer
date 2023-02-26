#[derive(Debug, Clone)]
pub enum PackVersionSpecifierRuntimeMeta {
	PackVersion(u8),
	MCVersion(String)
}
