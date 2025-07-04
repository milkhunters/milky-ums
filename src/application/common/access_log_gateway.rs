use async_trait::async_trait;

use crate::domain::models::access_log::AccessLog;
use crate::domain::models::user::UserId;

pub enum AccessLogGatewayError {
    Critical(String)
}

#[async_trait]
pub trait AccessLogReader {
    async fn get_user_records(&self, user_id: &UserId, limit: &u64, offset: &u64) -> Result<Vec<AccessLog>, AccessLogGatewayError>;

}

#[async_trait]
pub trait AccessLogWriter {
    async fn save(&self, data: &AccessLog) -> Result<(), AccessLogGatewayError>;
}

pub trait AccessLogGateway: AccessLogReader + AccessLogWriter + Sync + Send { }
