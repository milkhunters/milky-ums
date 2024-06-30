use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;
use serde_json::Value;

use crate::application::common::confirm_code::ConfirmCode;
use crate::application::common::email_sender::EmailSender;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::config::Extra;
use crate::domain::services::access::AccessService;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct SendConfirmCodeDTO {
    pub email: String,
}

pub struct SendConfirmCode<'a> {
    pub email_sender: &'a dyn EmailSender,
    pub confirm_code: &'a dyn ConfirmCode,
    pub extra: &'a Extra,
    pub user_reader: &'a dyn UserReader,
    pub validator: &'a ValidatorService,
    pub access_service: &'a AccessService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<SendConfirmCodeDTO, ()> for SendConfirmCode<'_> {
    async fn execute(&self, data: SendConfirmCodeDTO) -> Result<(), ApplicationError> {
        
        match self.access_service.ensure_can_send_confirm_code(
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
            validator_err_map.insert("email".to_string(), e.to_string());
        });

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let user = self.user_reader.get_user_by_email_not_sensitive(&data.email).await.ok_or(
            ApplicationError::NotFound(
                ErrorContent::Message("Пользователь не найден".to_string())
            )
        )?;
        
        let code = self.confirm_code.generate(&user.email).await.map_err(
            |error| ApplicationError::InvalidData(
                ErrorContent::Message(error.to_string())
            )
        )?;
        
        let context: BTreeMap<String, Value> = {
            let mut context = BTreeMap::new();
            context.insert("code".to_string(), Value::String(code.to_string()));
            context.insert("username".to_string(), Value::String(user.username));
            context.insert("company".to_string(), Value::String(self.extra.company.clone()));
            context.insert("company_url".to_string(), Value::String(self.extra.company_url.clone()));
            context
        };
        
        self.email_sender.send_template(
            &user.email,
            "Подтверждение операции смены данных",
            "confirm_code.html",
            Some(context),
            13,
            900, // 15 minutes
        ).await;
        
        Ok(())
    }
}
