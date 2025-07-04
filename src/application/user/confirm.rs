use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

use crate::application::common::code_confirmer::CodeConfirmer;
use crate::application::common::email_sender::EmailSender;
use crate::application::common::error::AppError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::error::ValidationError;
use crate::domain::models::user::UserState;
use crate::domain::services::access::ensure_can_confirm_user;
use crate::domain::services::validator::validate_email;

#[derive(Debug, Deserialize)]
pub struct ConfirmUserInput {
    pub email: String,
    pub code: u32
}

pub struct ConfirmUser<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub code_confirmer: &'interactor dyn CodeConfirmer,
    pub user_gateway: &'interactor dyn UserGateway,
    pub email_sender: &'interactor dyn EmailSender
}

impl Interactor<ConfirmUserInput, ()> for ConfirmUser<'_> {
    async fn execute(&self, data: ConfirmUserInput) -> Result<(), AppError> {
        ensure_can_confirm_user(&self.id_provider.permissions())?;

        let mut validator_err_map: HashMap<String, ValidationError> = HashMap::new();
        validate_email(&data.email).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });

        if !validator_err_map.is_empty() {
            return Err(AppError::Validation(validator_err_map));
        }

        let mut user = self.user_gateway.get_user_by_email_not_sensitive(&data.email).await?
            .ok_or_else(|| AppError::NotFound("email".into()))?;

        if !matches!(user.state, UserState::NotVerify) {
            return Err(AppError::Conflict("email".into()));
        }

        self.code_confirmer.confirm(&user.email, data.code).await?;
        
        user.update(
            user.username.clone(),
            user.email.clone(),
            UserState::Active,
            user.first_name,
            user.last_name,
            &user.hashed_password
        );
        
        self.user_gateway.save(&user).await?;

        let context: BTreeMap<String, String> = {
            let mut context = BTreeMap::new();
            context.insert("username".to_string(), user.username);
            // context.insert("company".to_string(), Value::String(self.extra.company.to_string()));
            // context.insert("company_url".to_string(), Value::String(self.extra.company_url.to_string()));
            context
        };

        self.email_sender.send_template(
            &user.email,
            "Подтверждение почты",
            "email_confirm_success.html",
            Some(context),
            13,
            3600
        ).await?;
        
        Ok(())
    }
}
