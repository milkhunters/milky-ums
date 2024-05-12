use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::cmd;
use redis::geo::Unit;
use uuid::Uuid;

use crate::application::common::session_gateway::{SessionReader, SessionWriter};
use crate::domain::models::session::Session;

pub struct SessionGateway {
    pub redis_pool: Box<Pool>
}

impl SessionGateway {
    pub fn new(redis_pool: Box<Pool>) -> Self {
        SessionGateway { redis_pool }
    }
}

#[async_trait]
impl SessionReader for SessionGateway {
    async fn get_session_by_id(&self, session_id: Uuid) -> Option<Session>{
        let mut conn = self.redis_pool.get().await.unwrap();
        match cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async::<_, String>(&mut conn)
            .await {
            Ok(value) => {
                Some(Session {
                    id: session_id.to_string(),
                    ip: "some ip".to_string(),
                    user_agent: "some user agent".to_string(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
            },
            Err(e) => {
                None
            }
        }
    }

    async fn get_sessions_by_user_id(&self, user_id: Uuid) -> Vec<Session> {
        todo!()
    }
}

#[async_trait]
impl SessionWriter for SessionGateway {
    async fn save_session(&self, session: &Session) -> Unit {
        todo!()
    }

    async fn delete_session(&self, session_id: Uuid) -> Unit {
        todo!()
    }
}