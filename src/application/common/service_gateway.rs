use async_trait::async_trait;

use crate::domain::models::service::{Service, ServiceId, ServiceTextId};

pub enum ServiceGatewayError {
    Critical(String)
}


#[async_trait]
pub trait ServiceReader {
    async fn get_service_by_id(&self, service_id: &ServiceId) -> Result<Option<Service>, ServiceGatewayError>;
    async fn get_services(&self, limit: &u64, offset: &u64) -> Result<Vec<Service>, ServiceGatewayError>;
    async fn get_services_by_text_id(&self, text_id: &ServiceTextId) -> Result<Option<Vec<Service>>, ServiceGatewayError>;
}

#[async_trait]
pub trait ServiceWriter {
    async fn save_service(&self, data: &Service) -> Result<(), ServiceGatewayError>;
}

#[async_trait]
pub trait ServiceRemover {
    async fn remove_service(&self, service_id: &ServiceId) -> Result<(), ServiceGatewayError>;
}


pub trait ServiceGateway: ServiceReader + ServiceWriter + ServiceRemover + Send + Sync {}