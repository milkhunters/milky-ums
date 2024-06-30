use std::collections::BTreeMap;

use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait EmailSender {
    async fn send(
        &self,
        to: &str,
        subject: &str,
        content: &str,
        content_type: &str,
        priority: u8,
        ttl: u32,
    );

    async fn send_template(
        &self,
        to: &str,
        subject: &str,
        template: &str,
        data: Option<BTreeMap<String, Value>>,
        priority: u8,
        ttl: u32,
    );
}
