use async_trait::async_trait;

pub enum HasherError {
    Critical(String)
}

#[async_trait]
pub trait Hasher: Send + Sync {
    async fn hash(&self, value: &[u8]) -> Result<[u8], HasherError>;
    async fn verify(&self, value: &[u8], hash: &[u8]) -> Result<bool, HasherError>;
}
