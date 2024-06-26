use std::collections::HashMap;

use serde::Deserialize;

use crate::application::common::confirm_code::ConfirmCode;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::models::user::UserState;
use crate::domain::services::access::AccessService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct ConfirmUserDTO {
    pub email: String,
    pub code: u32
}

pub struct ConfirmUser<'a> {
    pub confirm_code: &'a dyn ConfirmCode,
    pub user_gateway: &'a dyn UserGateway,
    pub user_service: &'a UserService,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<ConfirmUserDTO, ()> for ConfirmUser<'_> {
    async fn execute(&self, data: ConfirmUserDTO) -> Result<(), ApplicationError> {

        match self.access_service.ensure_can_confirm_user(
            &self.id_provider.is_auth(),
            &self.id_provider.permissions()
        ) {
            Ok(_) => (),
            Err(e) => return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message(e.to_string())
                )
            )
        };

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_email(&data.email).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }

        let user = self.user_gateway.get_user_by_email_not_sensitive(&data.email).await.ok_or(
            ApplicationError::NotFound(
                ErrorContent::Message("Пользователь не найден".to_string())
            )
        )?;

        match user.state {
            UserState::Active => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Пользователь уже активирован".to_string())
                )
            ),
            UserState::Inactive => (),
            _ => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Невозможно активировать пользователя".to_string())
                )
            )
        }

        self.confirm_code.confirm(&user.email, data.code).await.map_err(
            |error| ApplicationError::InvalidData(
                ErrorContent::Message(error.to_string())
            )
        )?;
        
        let new_user = self.user_service.update_user(
            user.clone(),
            user.username.clone(),
            user.email.clone(),
            UserState::Active,
            user.first_name,
            user.last_name,
            user.hashed_password
        );
        
        self.user_gateway.save_user(&new_user).await;
        
        Ok(())
    }
}
