use redis::cmd;
use uuid::Uuid;

use crate::application::common::session_gateway::SessionReader;
use crate::domain::models::session::Session;
use crate::adapters::redis::pool::RedisAsyncPool;

pub struct SessionGateway {
    pub redis_pool: RedisAsyncPool
}

impl<'a> SessionGateway {
    pub fn new(redis_pool: RedisAsyncPool) -> Self {
        SessionGateway { redis_pool }
    }
}

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
}