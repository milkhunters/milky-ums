use std::collections::HashMap;

use serde::Serialize;
use crate::domain::error::{DomainError, ValidationError};
use super::{
    access_log_gateway::AccessLogGatewayError,
    code_confirmer::CodeConfirmerError,
    email_sender::EmailSenderError,
    hasher::HasherError,
    permission_gateway::PermissionGatewayError,
    role_gateway::RoleGatewayError,
    service_gateway::ServiceGatewayError,
    session_gateway::SessionGatewayError,
    user_gateway::UserGatewayError
};


#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    Validation(HashMap<String, ValidationError>), // status, fields -> reasons ValidationError::*
    NotFound(String), // status, field -> reason NOT_FOUND
    AccessDenied, // status
    Conflict(String), // status, field -> reason CONFLICT
    Critical(String),
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::Validation(err) => AppError::Validation(
                HashMap::from([(err.0, err.1)])
            ),
            DomainError::Access => AppError::AccessDenied,
        }
    }
}

impl From<AccessLogGatewayError> for AppError {
    fn from(error: AccessLogGatewayError) -> Self {
        match error {
            AccessLogGatewayError::Critical(err) => AppError::Critical(
                format!("critical error in AccessLogGateway: {}", err)
            )
        }
    }
}

impl From<CodeConfirmerError> for AppError {
    fn from(error: CodeConfirmerError) -> Self {
        match error {
            CodeConfirmerError::Critical(err) => AppError::Critical(
                format!("critical error in CodeConfirmer: {}", err)
            )
        }
    }
}

impl From<EmailSenderError> for AppError {
    fn from(error: EmailSenderError) -> Self {
        match error {
            EmailSenderError::Critical(err) => AppError::Critical(
                format!("critical error in EmailSender: {}", err)
            )
        }
    }
}

impl From<HasherError> for AppError {
    fn from(error: HasherError) -> Self {
        match error {
            HasherError::Critical(err) => AppError::Critical(
                format!("critical error in Hasher: {}", err)
            )
        }
    }
}

impl From<PermissionGatewayError> for AppError {
    fn from(error: PermissionGatewayError) -> Self {
        match error {
            PermissionGatewayError::Critical(err) => AppError::Critical(
                format!("critical error in PermissionGateway: {}", err)
            )
        }
    }
}

impl From<RoleGatewayError> for AppError {
    fn from(error: RoleGatewayError) -> Self {
        match error {
            RoleGatewayError::Critical(err) => AppError::Critical(
                format!("critical error in RoleGateway: {}", err)
            )
        }
    }
}

impl From<ServiceGatewayError> for AppError {
    fn from(error: ServiceGatewayError) -> Self {
        match error {
            ServiceGatewayError::Critical(err) => AppError::Critical(
                format!("critical error in ServiceGateway: {}", err)
            )
        }
    }
}


impl From<SessionGatewayError> for AppError {
    fn from(error: SessionGatewayError) -> Self {
        match error {
            SessionGatewayError::Critical(err) => AppError::Critical(
                format!("critical error in SessionGateway: {}", err)
            )
        }
    }
}


impl From<UserGatewayError> for AppError {
    fn from(error: UserGatewayError) -> Self {
        match error {
            UserGatewayError::Critical(err) => AppError::Critical(
                format!("critical error in UserGateway: {}", err)
            )
        }
    }
}





#[derive(Debug)]
pub enum TokenError {
    Invalid(String),
    Expired,
    Critical(String),
}
