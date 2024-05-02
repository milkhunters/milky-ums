use argon2_async::Config;
use async_trait::async_trait;

use crate::application::common::hasher::Hasher;

pub struct Argon2Hasher {}

impl Argon2Hasher {
    pub fn new() -> Self {

        let config = Config {
            algorithm: Default::default(),
            version: Default::default(),
            secret_key: None,
            memory_cost: 2048,
            iterations: 64,
            parallelism: 4,
            output_length: Some(64),
        };

        futures::executor::block_on(async move {
            argon2_async::set_config(config).await;
        });

        Self {}
    }
}

#[async_trait]
impl Hasher for Argon2Hasher {
    async fn hash(&self, value: &str) -> String {
        argon2_async::hash(value.as_bytes()).await.unwrap()
    }

    async fn verify(&self, value: &str, hash: &str) -> bool {
        argon2_async::verify(value.to_string(), hash.to_string()).await.unwrap()
    }
}
