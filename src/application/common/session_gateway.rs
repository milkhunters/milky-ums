use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::service::ServiceTextId;
use crate::domain::models::session::{Session, SessionId, SessionTokenHash};
use crate::domain::models::user::{UserId, UserState};

pub enum SessionGatewayError {
    Critical(String)
}


#[async_trait]
pub trait SessionReader {
    async fn get_session(&self, session_id: &SessionId) -> Result<Option<Session>, SessionGatewayError>;
    async fn get_session_by_token_hash(
        &self, 
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, HashMap<ServiceTextId, Vec<PermissionTextId>>)>;
    async fn get_session_by_token_hash_from_cache(
        &self, 
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, HashMap<ServiceTextId, Vec<PermissionTextId>>)>;
    async fn get_user_sessions(&self, user_id: &UserId) -> Result<Vec<Session>, SessionGatewayError>;
}

#[async_trait]
pub trait SessionWriter {
    async fn save_session(&self, data: &Session) -> Result<(), SessionGatewayError>;
    async fn save_session_to_cache(
        &self, 
        data: &Session, 
        user_state: &UserState,
        permissions: &HashMap<ServiceTextId, Vec<PermissionTextId>>    
    ) -> Result<(), SessionGatewayError>;
}

#[async_trait]
pub trait SessionRemover {
    async fn remove(&self, session_id: &SessionId) -> Result<(), SessionGatewayError>;
    async fn remove_by_user_id(&self, user_id: &UserId) -> Result<(), SessionGatewayError>;
}


pub trait SessionGateway: SessionReader + SessionWriter + SessionRemover + Send + Sync {}