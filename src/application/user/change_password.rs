use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

use crate::application::common::email_sender::EmailSender;
use crate::application::common::error::AppError;
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::error::ValidationError;
use crate::domain::services::access::{ensure_can_update_user_self};
use crate::domain::services::validator::{validate_password};

#[derive(Debug, Deserialize)]
pub struct ChangePasswordInput {
    pub old_password: String,
    pub new_password: String,
}

pub struct ChangePassword<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub email_sender: &'interactor dyn EmailSender,
    pub user_gateway: &'interactor dyn UserGateway,
    pub password_hasher: &'interactor dyn Hasher
}

impl Interactor<ChangePasswordInput, ()> for ChangePassword<'_> {
    async fn execute(&self, data: ChangePasswordInput) -> Result<(), AppError> {
        ensure_can_update_user_self(
            self.id_provider.user_state(),
            self.id_provider.permissions()
        )?;

        if data.old_password == data.new_password {
            return Err(AppError::Critical("new_password".into()));
        }

        let mut validator_err_map: HashMap<String, ValidationError> = HashMap::new();
        validate_password(&data.new_password).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });
        
        if !validator_err_map.is_empty() {
            return Err(AppError::Validation(validator_err_map));
        }

        let mut user = self.user_gateway.get_user_by_id(self.id_provider.user_id()).await?
            .ok_or_else(|| AppError::Critical("User not found in ChangePassword".into()))?;

        let is_valid = self.password_hasher.verify(
            &data.old_password.as_bytes(), 
            &user.hashed_password
        ).await?;
        
        if !is_valid {
            return Err(AppError::Validation(HashMap::from([(
                "old_password".into(),
                ValidationError::Invalid
            )])));
        }
        
        let hash = self.password_hasher.hash(&data.new_password.as_bytes()).await?;

        user.update(
            user.username.clone(),
            user.email.clone(),
            user.state,
            user.first_name,
            user.last_name,
            &hash
        );
        
        self.user_gateway.save(&user).await?;
        
        let context: BTreeMap<String, String> = {
            let mut context = BTreeMap::new();
            context.insert("username".to_string(), user.username);
            context.insert("ip".to_string(), self.id_provider.ip().to_string());
            context.insert("change_time".to_string(), {
                let now = chrono::Utc::now();
                now.format("%d/%m/%Y %H:%M %Z").to_string()
            });
            context.insert("email".to_string(), user.email);
            // context.insert("company".to_string(), Value::String(self.extra.company.clone()));
            // context.insert("company_url".to_string(), Value::String(self.extra.company_url.clone()));
            // context.insert("reset_password_url".to_string(), Value::String(self.extra.reset_password_url.clone()));
            context
        };
        
        self.email_sender.send_template(
            &user.email,
            "Изменение пароля",
            "successfully_reset_password.html",
            Some(context),
            13,
            3600
        ).await?;
        
        Ok(())
    }
}
