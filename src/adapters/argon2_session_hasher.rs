use argon2::{password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
}, Argon2, Params};
use async_trait::async_trait;

use crate::application::common::hasher::Hasher;

pub struct Argon2SessionHasher {
    hasher: Argon2<'static>
}

impl Argon2SessionHasher {
    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::new(
                    2048,
                    1,
                    1,
                    Some(64)
                ).unwrap()
            )
        }
    }
}

#[async_trait]
impl Hasher for Argon2SessionHasher {
    async fn hash(&self, value: &str) -> String {
        let hasher = self.hasher.clone();
        let value = value.to_owned();
        let hash = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            hasher.hash_password(
                value.as_bytes(),
                &salt
            ).unwrap().to_string()
        }).await.unwrap();
        hash
    }

    async fn verify(&self, value: &str, hash: &str) -> bool {
        let hasher = self.hasher.clone();
        let value = value.to_owned();
        let hash = hash.to_owned();
        let result = tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash).unwrap();
            hasher.verify_password(
                value.as_bytes(),
                &parsed_hash
            ).is_ok()
        }).await.unwrap();
        result
    }
}
