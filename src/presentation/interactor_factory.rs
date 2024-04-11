
use crate::application::user::get_by_id::GetUserById;

pub trait InteractorFactory {
    fn get_user_by_id(&self) -> GetUserById;

}