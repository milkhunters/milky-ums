use crate::application::common::id_provider::IdProvider;
use crate::domain::exceptions::DomainError;

pub struct AccessService {
    permissions: Vec<String>,
}

impl AccessService {
    pub fn new(permissions: Vec<String>) -> Self {
        AccessService { permissions }
    }

    pub fn ensure_can_get_user_by_id(&self, identity: Box<dyn IdProvider>) -> Result<(), DomainError> {
        if self.permissions.contains(&identity.permission()) {
            Ok(())
        } else {
            Err(DomainError::AccessDenied)
        }
    }
}
