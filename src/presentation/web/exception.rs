use std::fmt::{Display, Formatter};
use actix_web::{error, HttpResponse, Result};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::json;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // stub implementation
    }
}

impl ApplicationError {
    fn error_rest_content(&self) -> (StatusCode, ErrorContent) {
        match *self {
            ApplicationError::InvalidData(ref content) => (StatusCode::BAD_REQUEST, content.clone()),
            ApplicationError::NotFound(ref content) => (StatusCode::NOT_FOUND, content.clone()),
            ApplicationError::Conflict(ref content) => (StatusCode::CONFLICT, content.clone()),
            ApplicationError::InternalError(ref content) => (StatusCode::INTERNAL_SERVER_ERROR, content.clone()),
        }
    }
}

impl error::ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        self.error_rest_content().0
    }

    fn error_response(&self) -> HttpResponse {
        let response = match self.error_rest_content().1 {
            ErrorContent::Message(msg) => json!({
                "error": msg
            }),
            ErrorContent::Map(map) => json!({
                "error": map.iter().map(|(field, message)| {
                    json!({
                        "field": field,
                        "message": message
                    })
                }).collect::<Vec<_>>()
            }),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
    }
}


pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&json!({
            "error": "Запрашиваемый ресурс не найден"
        })).unwrap()))
}