use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::cmd;
use uuid::Uuid;

use crate::application::common::session_gateway::SessionReader;
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
    async fn get_session_by_id(&self, session_id: Uuid) -> Result<Session, String> {
        let mut conn = self.redis_pool.get().await.unwrap();
        let value: String = cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async(&mut conn)
            .await.unwrap();

        Ok(Session {
            id: session_id.to_string(),
            ip: "some ip".to_string(),
            user_agent: "some user agent".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    async fn get_sessions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, String> {
        todo!()
    }
}