use crate::application::common::id_provider::IdProvider;
use crate::application::session::create::CreateSession;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionByUserId;
use crate::application::user::create::CreateUser;

pub trait InteractorFactory {
    fn get_user_by_id(&self) -> GetUserById;
    fn get_users_by_ids(&self) -> GetUsersByIds;
    fn get_user_range(&self) -> GetUserRange;
    fn create_user(&self) -> CreateUser;
    fn get_session_by_id(&self) -> GetSessionById;
    fn get_sessions_by_user_id(&self) -> GetSessionByUserId;
    fn create_session(&self, id_provider: &dyn IdProvider) -> CreateSession;
}