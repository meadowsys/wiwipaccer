use std::error::Error;

#[async_trait::async_trait]
pub trait ResourceProvider {
	async fn resource_for(&mut self, resource: &str) -> Result<String, Box<dyn Error>>;
}
