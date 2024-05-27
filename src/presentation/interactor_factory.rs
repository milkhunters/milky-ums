use crate::application::common::id_provider::IdProvider;
use crate::application::session::create::CreateSession;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionByUserId;
use crate::application::user::create::CreateUser;
use crate::application::user::get_self::GetUserSelf;
use crate::application::user::update::UpdateUser;
use crate::application::user::update_self::UpdateUserSelf;

pub trait InteractorFactory {
    fn get_user_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetUserById;
    fn get_users_by_ids(&self, id_provider: Box<dyn IdProvider>) -> GetUsersByIds;
    fn get_user_range(&self, id_provider: Box<dyn IdProvider>) -> GetUserRange;
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf;
    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser;
    fn update_user(&self, id_provider: Box<dyn IdProvider>) -> UpdateUser;
    fn update_user_self(&self, id_provider: Box<dyn IdProvider>) -> UpdateUserSelf;
    fn get_session_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionById;
    fn get_sessions_by_user_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionByUserId;
    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession;
}