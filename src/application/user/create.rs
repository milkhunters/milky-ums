use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::application::common::email_sender::EmailSender;
use crate::application::common::error::AppError;
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::role_gateway::RoleGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::error::ValidationError;
use crate::domain::models::user::{User, UserId, UserState};
use crate::domain::services::access::ensure_can_create_user;
use crate::domain::services::validator::{validate_email, validate_first_name, validate_last_name, validate_password, validate_username};

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResultDTO{
    id: UserId,
    username: String,
    email: String,
    state: UserState,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub struct CreateUser<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_gateway: &'interactor dyn UserGateway,
    pub role_gateway: &'interactor dyn RoleGateway,
    pub email_sender: &'interactor dyn EmailSender,
    pub password_hasher: &'interactor dyn Hasher,
}

impl Interactor<CreateUserDTO, CreateUserResultDTO> for CreateUser<'_> {
    async fn execute(&self, data: CreateUserDTO) -> Result<CreateUserResultDTO, AppError> {
        ensure_can_create_user(&self.id_provider.permissions())?;
        
        let mut validator_err_map: HashMap<String, ValidationError> = HashMap::new();
        validate_username(&data.username).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });

        validate_password(&data.password).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });

        validate_email(&data.email).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });

        if let Some(first_name) = &data.first_name {
            validate_first_name(first_name).unwrap_or_else(|e| {
                let (k, v) = e.to_validation();
                validator_err_map.insert(k, v);
            });
        }

        if let Some(last_name) = &data.last_name {
            validate_last_name(last_name).unwrap_or_else(|e| {
                let (k, v) = e.to_validation();
                validator_err_map.insert(k, v);
            });
        }

        if !validator_err_map.is_empty() {
            return Err(AppError::Validation(validator_err_map))
        }
        

        let (user_by_username, user_by_email) = tokio::try_join!(
            self.user_gateway.get_user_by_username_not_sensitive(&data.username),
            self.user_gateway.get_user_by_email_not_sensitive(&data.email)
        )?;

        if user_by_username.is_some() {
            return Err(AppError::Conflict("username".into()))
        }

        if user_by_email.is_some() {
            return Err(AppError::Conflict("email".into()))
        }

        
        let (hashed_password, default_role_id) = tokio::join!(
            self.password_hasher.hash(&data.password.as_bytes()),
            self.role_gateway.get_default_role()
        )?;

        let user = User::new(
            data.username,
            data.email,
            UserState::NotVerify,
            &hashed_password,
            data.first_name,
            data.last_name
        );

        self.user_gateway.save(&user).await?;
        self.role_gateway.link_role_to_user(&default_role_id, &user.id).await?;
        
        let mut context = BTreeMap::new();
        context.insert("username".to_string(), user.username.clone());
        
        self.email_sender.send_template(
            &user.email,
            "Регистрация на сайте",
            "registration.html",
            Some(context),
            13,
            3600
            
        ).await?;

        Ok(CreateUserResultDTO {
            id: user.id,
            username: user.username,
            email: user.email,
            state: user.state,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
