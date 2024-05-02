use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::user::{ User, UserState };

pub struct UserService { }

impl UserService {

    pub fn create_user(
        &self,
        username: String,
        email: String,
        hashed_password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, ApplicationError> {
        Ok(User {
            id: Uuid::new_v4(),
            username,
            email,
            first_name,
            last_name,
            state: UserState::Active,
            hashed_password,
            created_at: Default::default(),
            updated_at: None,
        })
    }
}
