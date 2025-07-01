use uuid::Uuid;

use crate::domain::{
    error::DomainError,
    models::{
        permission::PermissionTextId,
        session::SessionId,
        ums_permission::Permission,
        user::UserState
    }
};

pub fn ensure_can_create_user(
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    if !permissions.contains(&Permission::CreateUser.to_string()) {
        return Err(DomainError::Access)
    }
    Ok(())
}

pub fn ensure_can_get_user_self(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    if 
        permissions.contains(&Permission::GetUserSelf.to_string()) &&
        user_state.unwrap() != &UserState::NotVerify
    {
        return Ok(())
    }
    Err(DomainError::Access)
}

pub fn ensure_can_get_user(
    user_id: Option<&Uuid>,
    get_user_id: &Uuid,
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if permissions.contains(&Permission::GetUser.to_string()) {
        return Ok(())
    }
    
    if 
        permissions.contains(&Permission::GetUserSelf.to_string()) &&
        user_id.unwrap() == get_user_id &&
        user_state.unwrap() == &UserState::Active
    {
        return Ok(())
    }
    Err(DomainError::Access)
}

pub fn ensure_can_get_users(
    user_id: Option<&Uuid>,
    get_user_ids: &Vec<Uuid>,
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetUser.to_string()) {
        return Ok(())
    }

    if
        permissions.contains(&Permission::GetUserSelf.to_string()) && 
        get_user_ids.len() == 1 &&
        get_user_ids.contains(&user_id.unwrap()) &&
        user_state.unwrap() == &UserState::Active
    {
        return Ok(())
    }
    Err(DomainError::Access)
}

pub fn ensure_can_get_user_range(
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetUser.to_string()) {
        return Ok(())
    }
    
    Err(DomainError::Access)
}


pub fn ensure_can_update_user(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        user_state.unwrap() == &UserState::Active && 
        permissions.contains(&Permission::UpdateUser.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_update_user_self(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if
        user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::UpdateUserSelf.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_reset_password(
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if !permissions.contains(&Permission::ResetUserPassword.to_string())
    {
        return Err(DomainError::Access)
    }

    Ok(())
}


pub fn ensure_can_confirm_user(
    permissions: &Vec<PermissionTextId>
) -> Result<(), DomainError> {
    if !permissions.contains(&Permission::ConfirmUser.to_string()) {
        return Err(DomainError::Access)
    }
    Ok(())
}

pub fn ensure_can_send_confirm_code(
    permissions: &Vec<PermissionTextId>
) -> Result<(), DomainError> {
    if !permissions.contains(&Permission::SendConfirmCode.to_string()) {
        return Err(DomainError::Access)
    }
    Ok(())
}

pub fn ensure_can_delete_session(
    user_session_id: Option<&SessionId>,
    del_session_id: &SessionId,
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if permissions.contains(&Permission::DeleteSession.to_string()) ||
        (
            permissions.contains(&Permission::DeleteSessionSelf.to_string()) &&
            user_session_id.unwrap() == del_session_id &&
            user_state.unwrap() != &UserState::NotVerify
        )
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_delete_session_self(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if
        user_state.unwrap() != &UserState::NotVerify &&
        permissions.contains(&Permission::DeleteSessionSelf.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_create_session(
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if permissions.contains(&Permission::CreateSession.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_get_session(
    user_session_id: Option<&SessionId>,
    get_session_id: &SessionId,
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if
        permissions.contains(&Permission::GetSession.to_string()) ||
        (
            permissions.contains(&Permission::GetSessionSelf.to_string()) &&
            user_session_id.unwrap() == get_session_id &&
            user_state.unwrap() != &UserState::NotVerify
        )
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_get_sessions(
    user_id: Option<&Uuid>,
    get_user_id: &Uuid,
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetSession.to_string()) {
        return Ok(())
    }
    
    if
        permissions.contains(&Permission::GetSessionSelf.to_string()) &&
        get_user_id == user_id.unwrap() &&
        user_state.unwrap() == &UserState::Active
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}


pub fn ensure_can_get_session_self(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        permissions.contains(&Permission::GetSessionSelf.to_string()) &&
        user_state.unwrap() != &UserState::NotVerify
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_get_access_log_self(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetAccessLogSelf.to_string()) &&
        user_state.unwrap() == &UserState::Active 
    {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_get_access_log(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetAccessLog.to_string()) &&
        user_state.unwrap() == &UserState::Active
    {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_create_role(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::CreateRole.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_set_default_role(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if
    user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::SetDefaultRole.to_string())
    {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_delete_role(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::DeleteRole.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_get_role(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::GetRole.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_link_role_user(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::LinkUserRole.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_get_user_roles(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if
    user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::GetUserRole.to_string())
    {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_update_role(
    user_state: Option<&UserState>,
    permissions: &Vec<String>
) -> Result<(), DomainError> {
    
    if 
        user_state.unwrap() == &UserState::Active &&
        permissions.contains(&Permission::UpdateRole.to_string())
    {
        return Ok(())
    }
    
    Err(DomainError::Access)
}

pub fn ensure_can_get_permissions(
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetPermission.to_string()) {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_update_permission(
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::UpdatePermission.to_string()) {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_link_permission(
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::LinkRolePermission.to_string()) {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_get_service(
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::GetService.to_string()) {
        return Ok(())
    }

    Err(DomainError::Access)
}

pub fn ensure_can_update_service(
    permissions: &Vec<String>
) -> Result<(), DomainError> {

    if permissions.contains(&Permission::UpdateService.to_string()) {
        return Ok(())
    }

    Err(DomainError::Access)
}

