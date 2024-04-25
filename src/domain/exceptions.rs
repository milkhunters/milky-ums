
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum DomainError {
    #[display(fmt = "Ошибка аутентификации.")]
    AuthenticationError,

    #[display(fmt = "У Вас нет доступа к этому ресурсу.")]
    AccessDenied,
}
