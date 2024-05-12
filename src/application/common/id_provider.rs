use uuid::Uuid;
use crate::domain::models::user::UserState;

pub trait IdProvider {
    fn user_id(&self) -> Uuid;
    fn user_state(&self) -> UserState;
    fn permissions(&self) -> Vec<String>;
    fn user_agent(&self) -> String;
    fn ip(&self) -> String;
    fn is_auth(&self) -> bool;
}
