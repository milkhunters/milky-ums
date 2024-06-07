use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use deadpool_redis::Pool;
use futures::future::select;
use redis::cmd;
use sea_orm::{ActiveEnum, DbBackend, DbConn, EntityOrSelect, EntityTrait, FromQueryResult, JoinType, JsonValue, ModelTrait, QueryFilter, QuerySelect, QueryTrait, Related, RelationTrait, SelectColumns, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapters::database::models::{permissions, role_permissions, role_user, roles, sessions, users};
use crate::adapters::database::models::prelude::Users;
use crate::adapters::database::models::services::Relation::Permissions;
use crate::application::common::session_gateway::{SessionGateway as SessionGatewayTrait, SessionReader, SessionRemover, SessionWriter};
use crate::domain::models::permission::PermissionTextId;
use crate::domain::models::role::RoleId;
use crate::domain::models::session::{
    Session, 
    SessionId,
    SessionTokenHash
};
use crate::domain::models::user::UserState;

pub struct SessionGateway {
    cache_redis_pool: Box<Pool>,
    db: Box<DbConn>,
}

impl SessionGateway {
    pub fn new(
        redis_pool: Box<Pool>,
        db: Box<DbConn>
    ) -> Self {
        SessionGateway {
            cache_redis_pool: redis_pool,
            db
        }
    }
}

#[async_trait]
impl SessionReader for SessionGateway {
    async fn get_session(&self, session_id: &SessionId) -> Option<Session> {
        match sessions::Entity::find_by_id(session_id.clone())
            .one(&*self.db)
            .await.unwrap() {
            Some(model) => Some(map_session_model_to_domain(model)),
            None => None
        }
    }

    async fn get_session_by_token_hash(
        &self,
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, Vec<(RoleId, Vec<PermissionTextId>)>)> {
        // let session_vec= sessions::Entity::find()
        //     .select_with(permissions::Entity)
        //     .select_column(roles::Column::Id)
        //     .filter(Expr::col(sessions::Column::TokenHash).eq(token_hash.as_str()))
        //     .join(
        //         JoinType::LeftJoin,
        //         sessions::Relation::Users.def()
        //     )
        //     .join(
        //         JoinType::LeftJoin,
        //         users::Relation::RoleUser.def()
        //     )
        //     .join(
        //         JoinType::LeftJoin,
        //         role_user::Relation::Roles.def()
        //     )
        //     .join(
        //         JoinType::LeftJoin,
        //         roles::Relation::RolePermissions.def()
        //     )
        //     .join(
        //         JoinType::LeftJoin,
        //         role_permissions::Relation::Permissions.def()
        //     )
        //     .all(&*self.db)
        //     .await.unwrap();
        let session_vec= sessions::Entity::find()
            .filter(Expr::col(sessions::Column::TokenHash).eq(token_hash.as_str()))
            .all(&*self.db)
            .await.unwrap();
        
        if session_vec.is_empty() {
            return None;
        }
        
        if session_vec.len() > 1 {
            panic!("Multiple sessions with the same token hash");
        }
        
        let session = session_vec.first().unwrap();
        
        let user = session.find_related(Users).one(&*self.db).await.unwrap().unwrap();
        let user_state = UserState::from_str(user.state.to_value().as_str()).unwrap();
        
        let roles = user.find_related(roles::Entity).all(&*self.db).await.unwrap();
        
        let subquery = user.find_related(role_user::Entity).select_column(role_user::Column::RoleId).as_query();
        let permissions = role_permissions::Entity::find()
            .filter(Expr::col(role_permissions::Column::RoleId).in_subquery(
                user.find_related(role_user::Entity)
                    .select_column(role_user::Column::RoleId)
                    .as_query().to_owned()
            ))
            .join(
                JoinType::LeftJoin,
                role_permissions::Relation::Permissions.def()
            )
            .join(
                JoinType::LeftJoin,
                role_permissions::Relation::Roles.def()
            )
            .all(&*self.db)
            .await.unwrap();

        let mut roles_permissions: Vec<(RoleId, Vec<PermissionTextId>)> = Vec::new();
        None
        

    }

    async fn get_session_by_token_hash_from_cache(
        &self,
        token_hash: &SessionTokenHash
    ) -> Option<(Session, UserState, Vec<(RoleId, Vec<PermissionTextId>)>)> {
        let mut conn = self.cache_redis_pool.get().await.unwrap();
        match cmd("GET")
            .arg(token_hash.as_str())
            .query_async::<_, String>(&mut conn)
            .await {
            Ok(value) => {
                Some(
                    serde_json::from_str::<(
                        Session,
                        UserState,
                        Vec<(RoleId, Vec<PermissionTextId>)>
                    )>(value.as_str()).unwrap()
                )
            },
            Err(_) => {
                None
            }
        }
    }

    async fn get_user_sessions(&self, user_id: &Uuid) -> Vec<Session> {
        let sessions: Vec<sessions::Model> = sessions::Entity::find().filter(
            Expr::col(sessions::Column::UserId).eq(user_id.to_string())
        )
            .all(&*self.db)
            .await
            .unwrap();

        sessions.iter().map(
            |model| map_session_model_to_domain(model.clone())
        ).collect()
    }
}

#[async_trait]
impl SessionWriter for SessionGateway {
    async fn save_session(&self, data: &Session) {
        let session_model = sessions::ActiveModel {
            id: Set(data.id),
            token_hash: Set(data.token_hash.clone()),
            user_id: Set(data.user_id),
            ip: Set(data.ip.clone()),
            user_agent: Set(data.user_agent.clone()),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at.clone())
        };

        let mut conn = self.cache_redis_pool.get().await.unwrap();

        cmd("SET")
            .arg(data.token_hash.as_str())
            .arg(serde_json::to_string(&data).unwrap().as_str())
            .query_async::<_, ()>(&mut conn)
            .await.unwrap();
        
        match sessions::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                sessions::Entity::update(session_model).exec(&*self.db).await.unwrap();
            },
            None => {
                sessions::Entity::insert(session_model).exec(&*self.db).await.unwrap();
            }
        }
    }

    async fn save_session_to_cache(
        &self,
        data: &Session,
        user_state: &UserState,
        roles: &Vec<(RoleId, Vec<PermissionTextId>)>
    ) {
        let mut conn = self.cache_redis_pool.get().await.unwrap();
        
        let serde_json = serde_json::to_string(&(
            data.clone(),
            user_state.clone(),
            roles.clone()
        )).unwrap();
        
        cmd("SET")
            .arg(data.token_hash.as_str())
            .arg(serde_json.as_str())
            .query_async::<_, ()>(&mut conn)
            .await.unwrap();
    }
}

#[async_trait]
impl SessionRemover for SessionGateway {
    async fn remove_session(&self, session_id: &SessionId) {
        sessions::Entity::delete_by_id(session_id.clone())
            .exec(&*self.db)
            .await
            .unwrap();
    }
    
}

impl SessionGatewayTrait for SessionGateway {}

fn map_session_model_to_domain(model: sessions::Model) -> Session {
    Session {
        id: model.id,
        token_hash: model.token_hash,
        user_id: model.user_id,
        ip: model.ip,
        user_agent: model.user_agent,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}
