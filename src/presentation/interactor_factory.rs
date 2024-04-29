
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionByUserId;

pub trait InteractorFactory {
    fn get_user_by_id(&self) -> GetUserById;
    fn get_users_by_ids(&self) -> GetUsersByIds;
    fn get_user_range(&self) -> GetUserRange;
    fn get_session_by_id(&self) -> GetSessionById;
    fn get_sessions_by_user_id(&self) -> GetSessionByUserId;
}