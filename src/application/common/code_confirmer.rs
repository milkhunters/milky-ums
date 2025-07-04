use async_trait::async_trait;

pub enum CodeConfirmerError {
    Critical(String)
}

/// **CodeConfirmer** - интерфейс для работы с кодами подтверждения
/// 
/// Позволяет генерировать и подтверждать коды, идентифицируя по уникальному
/// ключу. Может использоваться для подтверждения email, телефона и т.д.
/// 
/// Сохраняет состояния.
#[async_trait]
pub trait CodeConfirmer {
    async fn confirm(&self, key: &str, code: u32) -> Result<(), CodeConfirmerError>;
    
    /// **Generate** - генерация кода подтверждения
    /// 
    /// Генерирует код подтверждения по уникальному ключу.
    /// Результат возвращается в виде шестизначного числа.
    async fn generate(&self, key: &str) -> Result<u32, CodeConfirmerError>;
}
