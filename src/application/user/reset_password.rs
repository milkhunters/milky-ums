use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

use crate::application::common::code_confirmer::CodeConfirmer;
use crate::application::common::email_sender::EmailSender;
use crate::application::common::error::AppError;
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionRemover;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::error::ValidationError;
use crate::domain::models::user::User;
use crate::domain::services::access::ensure_can_reset_password;
use crate::domain::services::validator::{validate_email, validate_password};

#[derive(Debug, Deserialize)]
pub struct ResetPasswordDTO {
    pub email: String,
    pub new_password: String,
    pub code: u32,
}

pub struct ResetPassword<'interactor> {
    pub id_provider: Box<dyn IdProvider>,
    pub email_sender: &'interactor dyn EmailSender,
    pub confirm_code: &'interactor dyn CodeConfirmer,
    pub user_gateway: &'interactor dyn UserGateway,
    pub password_hasher: &'interactor dyn Hasher,
    pub session_remover: &'interactor dyn SessionRemover,
}

impl Interactor<ResetPasswordDTO, ()> for ResetPassword<'_> {
    async fn execute(&self, data: ResetPasswordDTO) -> Result<(), AppError> {
        ensure_can_reset_password(self.id_provider.permissions())?;

        let mut validator_err_map: HashMap<String, ValidationError> = HashMap::new();
        validate_email(&data.email).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });

        validate_password(&data.new_password).unwrap_or_else(|e| {
            let (k, v) = e.to_validation();
            validator_err_map.insert(k, v);
        });

        if !validator_err_map.is_empty() {
            return Err(AppError::Validation(validator_err_map));
        }
        
        let (mut user, mut code_confirm, mut hashed_password) = tokio::join!(
            self.user_gateway.get_user_by_email_not_sensitive(&data.email),
            self.confirm_code.confirm(&data.email,data.code),
            self.password_hasher.hash(&data.new_password.as_bytes())
        )?;
        
        user = user?.ok_or_else(|| AppError::NotFound("email".into()))?;
        code_confirm = code_confirm?;
        hashed_password = hashed_password?;
        
        let new_user = User::new(
            user.username.clone(),
            user.email.clone(),
            user.state,
            &hashed_password,
            user.first_name,
            user.last_name,
        );
        
        let (res, res2) = tokio::join!(
            self.user_gateway.save(&new_user),
            self.session_remover.remove_user_sessions(&user.id)
        );
        
        res?;
        res2?;
        
        let mut context: BTreeMap<String, String> = BTreeMap::new();
        context.insert("username".to_string(), user.username.clone());
        context.insert("ip".to_string(), self.id_provider.ip().to_string());
        context.insert("change_time".to_string(), chrono::Utc::now().format("%d/%m/%Y %H:%M %Z").to_string());
        context.insert("email".to_string(), user.email.clone());
        
        self.email_sender.send_template(
            &user.email,
            "Сброс пароля",
            "successfully_reset_password.html",
            Some(context),
            13,
            3600
        ).await?;
        
        Ok(())
    }
}
