use async_trait::async_trait;
use deadpool_redis::Pool;
use redis::cmd;
use uuid::Uuid;

use crate::application::common::session_gateway::{SessionReader, SessionWriter, SessionGateway as SessionGatewayTrait};
use crate::domain::models::session::{Session, SessionId};

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
    async fn get_session(&self, session_id: &SessionId) -> Option<Session> {
        let mut conn = self.redis_pool.get().await.unwrap();
        match cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async::<_, String>(&mut conn)
            .await {
            Ok(value) => {
                Some(Session {
                    id: session_id.to_string(),
                    user_id: Uuid::new_v4(),
                    ip: "some ip".to_string(),
                    user_agent: "some user agent".to_string(),
                    created_at: chrono::Utc::now(),
                    updated_at: Some(chrono::Utc::now()),
                })
            },
            Err(e) => {
                None
            }
        }
    }

    async fn get_sessions(&self, user_id: &Uuid) -> Vec<Session> {
        let mut conn = self.redis_pool.get().await.unwrap();
        match cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async::<_, String>(&mut conn)
            .await {
            Ok(value) => {
                vec![Session {
                    id: Uuid::new_v4().to_string(),
                    user_id: user_id.clone(),
                    ip: "some ip".to_string(),
                    user_agent: "some user agent".to_string(),
                    created_at: chrono::Utc::now(),
                    updated_at: Some(chrono::Utc::now()),
                }]
            },
            Err(e) => {
                vec![]
            }
        }
    }
}

#[async_trait]
impl SessionWriter for SessionGateway {
    async fn save_session(&self, data: &Session) {
        let mut conn = self.redis_pool.get().await.unwrap();
        let _: () = cmd("SET")
            .arg(&[data.id.as_str(), serde_json::to_string(data).unwrap().as_str()])
            .query_async(&mut conn)
            .await.unwrap();
    }

    async fn delete_session(&self, session_id: &SessionId) {
        let mut conn = self.redis_pool.get().await.unwrap();
        let _: () = cmd("DEL")
            .arg(&[session_id])
            .query_async(&mut conn)
            .await.unwrap();
    }
}

impl SessionGatewayTrait for SessionGateway {}