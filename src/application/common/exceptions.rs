use std::collections::HashMap;
use std::fmt::Display;
use derive_more::{Display, Error};
use serde::Serialize;


#[derive(Debug, Serialize, Clone)]
pub enum ErrorContent {
    Message(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Serialize, Clone)]
pub enum ApplicationError {
    InvalidData(ErrorContent),
    NotFound(ErrorContent),
    Conflict(ErrorContent),
    InternalError(ErrorContent),
}
