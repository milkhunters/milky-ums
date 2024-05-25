use uuid::Uuid;
use crate::domain::exceptions::DomainError;
use crate::domain::models::session::SessionId;
use crate::domain::models::ums_permission::UMSPermission;
use crate::domain::models::user::UserState;

pub struct AccessService {}

impl AccessService {
    
    pub fn ensure_can_create_user(
        &self,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !permissions.contains(UMSPermission::CreateUser.as_str()) {
            return Err(DomainError::AccessDenied)
        }
        Ok(())
    }

    pub fn ensure_can_get_user_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            permissions.contains(UMSPermission::GetUserSelf.as_str()) &&
            user_state.unwrap() != &UserState::Inactive
        {
            return Ok(())
        }
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_user(
        &self,
        is_auth: &bool,
        user_id: Option<&Uuid>,
        get_user_id: &Uuid,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if permissions.contains(UMSPermission::GetUser.as_str()) {
            return Ok(())
        }
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            permissions.contains(UMSPermission::GetUserSelf.as_str()) &&
            user_id.unwrap() == get_user_id &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_users(
        &self,
        is_auth: &bool,
        user_id: Option<&Uuid>,
        get_user_ids: &Vec<Uuid>,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if permissions.contains(UMSPermission::GetUser.as_str()) {
            return Ok(())
        }

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if
            permissions.contains(UMSPermission::GetUserSelf.as_str()) && 
            get_user_ids.len() == 1 &&
            get_user_ids.contains(&user_id.unwrap()) &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_user_range(
        &self,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if permissions.contains(UMSPermission::GetUser.as_str()) {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }


    pub fn ensure_can_update_user(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            user_state.unwrap() == &UserState::Active && 
            permissions.contains(UMSPermission::UpdateUser.as_str())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_update_user_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {

        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(UMSPermission::UpdateUserSelf.as_str())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_delete_session(
        &self,
        is_auth: &bool,
        user_session_id: Option<&SessionId>,
        del_session_id: &SessionId,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            permissions.contains(UMSPermission::DeleteSession.as_str()) ||
            (
                permissions.contains(UMSPermission::DeleteSessionSelf.as_str()) &&
                user_session_id.unwrap() == del_session_id &&
                user_state != &UserState::Inactive
            )
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_delete_session_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            user_state.unwrap() == &UserState::Active &&
            permissions.contains(UMSPermission::DeleteSessionSelf.as_str())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_create_session(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
            
        if 
            is_auth &&
            permissions.contains(UMSPermission::CreateSession.as_str()) && 
            user_state.unwrap() != UserState::Inactive 
        {
            return Ok(())
        }
        
        if
            !is_auth &&
            permissions.contains(UMSPermission::CreateSession.as_str())
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
    pub fn ensure_can_get_session(
        &self,
        is_auth: &bool,
        user_session_id: Option<&SessionId>,
        get_session_id: &SessionId,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if
            permissions.contains(UMSPermission::GetSession.as_str()) ||
            (
                permissions.contains(UMSPermission::GetSessionSelf.as_str()) &&
                user_session_id.unwrap() == get_session_id &&
                user_state.unwrap() != &UserState::Inactive
            )
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }

    pub fn ensure_can_get_sessions(
        &self,
        is_auth: &bool,
        user_id: Option<&Uuid>,
        get_user_id: &Uuid,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }

        if permissions.contains(UMSPermission::GetSession.as_str()) {
            return Ok(())
        }
        
        if
            permissions.contains(UMSPermission::GetSessionSelf.as_str()) &&
            get_user_id == user_id.unwrap() &&
            user_state.unwrap() == &UserState::Active
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    

    pub fn ensure_can_get_session_self(
        &self,
        is_auth: &bool,
        user_state: Option<&UserState>,
        permissions: &Vec<String>
    ) -> Result<(), DomainError> {
        if !is_auth {
            return Err(DomainError::AuthorizationRequired)
        }
        
        if 
            permissions.contains(UMSPermission::GetSessionSelf.as_str()) &&
            user_state.unwrap() != &UserState::Inactive
        {
            return Ok(())
        }
        
        Err(DomainError::AccessDenied)
    }
    
}
