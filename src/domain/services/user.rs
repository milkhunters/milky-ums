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

    pub fn update_user(
        &self,
        user: User,
        new_username: String,
        new_email: String,
        new_state: UserState,
        new_first_name: Option<String>,
        new_last_name: Option<String>,
    ) -> Result<User, ApplicationError> {
        Ok(User {
            username: new_username,
            email: new_email,
            state: new_state,
            first_name: new_first_name,
            last_name: new_last_name,
            updated_at: Some(chrono::Utc::now()),
            ..user
        })
    }

    pub fn update_user_self(
        &self,
        user: User,
        new_username: String,
        new_first_name: Option<String>,
        new_last_name: Option<String>,
    ) -> Result<User, ApplicationError> {
        Ok(User {
            username: new_username,
            first_name: new_first_name,
            last_name: new_last_name,
            updated_at: Some(chrono::Utc::now()),
            ..user
        })
    }
    
}
