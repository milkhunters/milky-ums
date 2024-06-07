use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::role::RoleId;

use crate::domain::models::session::{Session, SessionId, SessionTokenHash};
use crate::domain::models::user::UserState;

#[async_trait]
pub trait SessionReader {
    async fn get_session(&self, session_id: &SessionId) -> Option<Session>;
    async fn get_session_by_token_hash(
        &self, 
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, Vec<(RoleId, Vec<PermissionTextId>)>)>;
    async fn get_session_by_token_hash_from_cache(
        &self, 
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, Vec<(RoleId, Vec<PermissionTextId>)>)>;
    async fn get_user_sessions(&self, user_id: &Uuid) -> Vec<Session>;
}

#[async_trait]
pub trait SessionWriter {
    async fn save_session(&self, data: &Session);
    async fn save_session_to_cache(
        &self, 
        data: &Session, 
        user_state: &UserState, 
        roles: &Vec<(RoleId, Vec<PermissionTextId>)>    
    );
}

#[async_trait]
pub trait SessionRemover {
    async fn remove_session(&self, session_id: &SessionId);
}


pub trait SessionGateway: SessionReader + SessionWriter + SessionRemover {}