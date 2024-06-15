use crate::domain::models::session::SessionId;
use crate::domain::models::user::{UserId, UserState};

pub trait IdProvider {
    fn session_id(&self) -> Option<&SessionId>;
    fn user_id(&self) -> Option<&UserId>;
    fn user_state(&self) -> Option<&UserState>;
    fn permissions(&self) -> &Vec<String>;
    fn user_agent(&self) -> &str;
    fn ip(&self) -> &str;
    fn is_auth(&self) -> &bool;
}
