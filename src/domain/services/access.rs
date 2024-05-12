use uuid::Uuid;
use crate::domain::exceptions::DomainError;
use crate::domain::models::permission::Permission;
use crate::domain::models::user::UserState;

pub struct AccessService {}

impl AccessService {

    pub fn ensure_can_get_user_self(
        &self,
        user_id: &Uuid,
        update_user_id: &Uuid,
        user_state: &UserState,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !(
            permissions.contains(Permission::GetUserSelf.as_str()) &&
            user_id == update_user_id &&
            user_state == &UserState::Active
        ) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_get_user(
        &self,
        user_state: &UserState,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !(
            permissions.contains(Permission::GetUser.as_str()) &&
            user_state == &UserState::Active
        ) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_update_user(
        &self,
        user_state: &UserState,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !(
            permissions.contains(Permission::UpdateUser.as_str()) &&
            user_state == &UserState::Active
        ) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_update_user_self(
        &self,
        user_id: &Uuid,
        update_user_id: &Uuid,
        user_state: &UserState,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !(
            permissions.contains(Permission::UpdateUserSelf.as_str()) &&
            user_id == update_user_id &&
            user_state == &UserState::Active
        ) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_delete_session_by_id(
        &self,
        user_session_id: &Uuid,
        del_session_id: &Uuid,
        user_state: &UserState,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !(
            permissions.contains(Permission::DeleteSession.as_str()) &&
            user_session_id == del_session_id &&
            user_state == &UserState::Active
        ) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_delete_session_self(
        &self,
        user_session_id: &Uuid,
        user_state: &UserState,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !(
            permissions.contains(Permission::DeleteSessionSelf.as_str()) &&
            user_session_id == user_session_id &&
            user_state == &UserState::Active
        ) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }
}
