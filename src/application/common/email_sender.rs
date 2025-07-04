use std::collections::BTreeMap;

use async_trait::async_trait;


pub enum EmailSenderError {
    Critical(String)
}

#[async_trait]
pub trait EmailSender {
    async fn send_template(
        &self,
        to: &str,
        subject: &str,
        template: &str,
        data: Option<BTreeMap<String, String>>,
        priority: u8,
        ttl: u32,
    ) -> Result<(), EmailSenderError>;
}
