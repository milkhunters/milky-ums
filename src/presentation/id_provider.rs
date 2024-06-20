use actix_web::HttpRequest;

use crate::adapters::auth::header::IdHeaderProvider;
use crate::application::common::id_provider::IdProvider;
use crate::domain::models::service::ServiceTextId;

pub fn get_id_provider(
    service_name: &ServiceTextId,
    req: &HttpRequest
) -> Box<dyn IdProvider> {
    
    let headers = req.headers();
    
    Box::new(IdHeaderProvider::new(
        service_name,
        match headers.get("payload") {
            Some(value) => Some(value.to_str().unwrap().to_string()),
            None => None
        },
        match headers.get("User-Agent") {
            Some(value) => value.to_str().unwrap_or(""),
            None => ""
        },
        req.connection_info().realip_remote_addr().unwrap()
    ))
}
