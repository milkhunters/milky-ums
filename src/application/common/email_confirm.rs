use async_trait::async_trait;


#[async_trait]
pub trait EmailConfirm {
    async fn confirm_email(&self, email: &str, code: u32) -> Result<bool, String>;
    async fn send_code(&self, email: &str) -> Result<(), String>;
}