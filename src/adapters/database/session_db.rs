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
            .arg(session_id.as_str())
            .query_async::<_, String>(&mut conn)
            .await {
            Ok(value) => {
                let model: Session = serde_json::from_str(value.as_str()).unwrap();
                Some(model)
            },
            Err(e) => {
                None
            }
        }
    }

    async fn get_sessions(&self, user_id: &Uuid) -> Vec<Session> {
        let mut conn = self.redis_pool.get().await.unwrap();
        let values = cmd("SMEMBERS")
            .arg(user_id.to_string().as_str())
            .query_async::<_, Vec<SessionId>>(&mut conn)
            .await.unwrap_or_else(|e| {
            vec![]
        });
        
        return match cmd("MGET")
            .arg(values)
            .query_async::<_, Vec<String>>(&mut conn)
            .await {
            Ok(values) => {
                values.iter().map(|value| {
                    let model: Session = serde_json::from_str(value.as_str()).unwrap();
                    model
                }).collect()
            },
            Err(e) => {
                vec![]
            }
        };
    }
}

#[async_trait]
impl SessionWriter for SessionGateway {
    async fn save_session(&self, data: &Session) {
        let mut conn = self.redis_pool.get().await.unwrap();
        
        // todo: gather all commands in a transaction
        let _: () = cmd("SET")
            .arg(data.id.as_str())
            .arg(serde_json::to_string(data).unwrap().as_str())
            .query_async(&mut conn)
            .await.unwrap();
        let _: () = cmd("SADD")
            .arg(data.user_id.to_string().as_str())
            .arg(data.id.as_str())
            .query_async(&mut conn)
            .await.unwrap();
    }

    async fn delete_session(&self, session_id: &SessionId, user_id: &Uuid) {
        let mut conn = self.redis_pool.get().await.unwrap();
        
        // todo: gather all commands in a transaction
        let _: () = cmd("DEL")
            .arg(session_id.as_str())
            .query_async(&mut conn)
            .await.unwrap();
        let _: () = cmd("SREM")
            .arg(user_id.to_string().as_str())
            .arg(session_id.as_str())
            .query_async(&mut conn)
            .await.unwrap();
    }
}

impl SessionGatewayTrait for SessionGateway {}