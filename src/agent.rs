use crate::result::Result;
use async_trait::async_trait;
#[async_trait]
pub trait Agent {
    /// Get one image
    async fn get(&self) -> Result<String>;
    /// Get many images
    async fn get_many(&self) -> Result<Vec<String>>;
    /// Get Random one
    async fn get_random(&self) -> Result<String>;
}
