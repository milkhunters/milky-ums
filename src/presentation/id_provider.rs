use std::str::FromStr;
use actix_web::HttpRequest;
use crate::adapters::auth::header::IdHeaderProvider;
use crate::application::common::id_provider::IdProvider;
use crate::domain::models::session::SessionId;

pub fn get_id_provider(
    req: &HttpRequest
) -> Box<dyn IdProvider> {
    
    let headers = req.headers();
    
    Box::new(IdHeaderProvider::new(
        match headers.get("session_token") {
            Some(value) => Some(
                SessionId::from_str(value.to_str().unwrap()).unwrap()
            ),
            None => None
        },
        match headers.get("payload") {
            Some(value) => Some(value.to_str().unwrap().to_string()),
            None => None
        },
        match headers.get("User-Agent") {
            Some(value) => value.to_str().unwrap_or("").to_string(),
            None => "".to_string()
        },
        req.connection_info().host().to_string(), // todo: check if this is correct
    ))
}
