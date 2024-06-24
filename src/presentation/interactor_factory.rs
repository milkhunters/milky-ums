use crate::application::common::id_provider::IdProvider;
use crate::application::service::sync::ServiceSync;
use crate::application::session::create::CreateSession;
use crate::application::session::delete::DeleteSession;
use crate::application::session::delete_self::DeleteSessionSelf;
use crate::application::session::extract_payload::EPSession;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionsByUserId;
use crate::application::session::get_self::GetSessionSelf;
use crate::application::user::change_password::ChangePassword;
use crate::application::user::confirm::ConfirmUser;
use crate::application::user::create::CreateUser;
use crate::application::user::get_self::GetUserSelf;
use crate::application::user::send_confirm_code::SendConfirmCode;
use crate::application::user::update::UpdateUser;
use crate::application::user::update_self::UpdateUserSelf;

pub trait InteractorFactory: Send + Sync {
    fn get_user_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetUserById;
    fn get_users_by_ids(&self, id_provider: Box<dyn IdProvider>) -> GetUsersByIds;
    fn get_user_range(&self, id_provider: Box<dyn IdProvider>) -> GetUserRange;
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf;
    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser;
    fn update_user(&self, id_provider: Box<dyn IdProvider>) -> UpdateUser;
    fn update_user_self(&self, id_provider: Box<dyn IdProvider>) -> UpdateUserSelf;
    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession;
    fn delete_session(&self, id_provider: Box<dyn IdProvider>) -> DeleteSession;
    fn delete_self_session(&self, id_provider: Box<dyn IdProvider>) -> DeleteSessionSelf;
    fn get_session_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionById;
    fn get_sessions_by_user_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionsByUserId;
    fn get_sessions_self(&self, id_provider: Box<dyn IdProvider>) -> GetSessionSelf;
    fn extract_payload(&self, id_provider: Box<dyn IdProvider>) -> EPSession;
    fn sync_service(&self) -> ServiceSync;
    fn send_confirm_code(&self, id_provider: Box<dyn IdProvider>) -> SendConfirmCode;
    fn confirm_user(&self, id_provider: Box<dyn IdProvider>) -> ConfirmUser;
    fn change_password(&self, id_provider: Box<dyn IdProvider>) -> ChangePassword;
}