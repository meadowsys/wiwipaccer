#[derive(Debug, Clone, serde::Serialize)]
pub enum PackVersionSpecifierRuntimeMeta {
	PackVersion(u8),
	MCVersion(String)
}
