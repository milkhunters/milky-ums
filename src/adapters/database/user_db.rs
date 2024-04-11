use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::adapters::database::connector::DbConnector;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user;

pub struct UserGateway {
    pub db: DbConnector,
}

impl<'a> UserGateway {
    pub fn new(db: DbConnector) -> Self {
        UserGateway { db }
    }
}

impl UserReader for UserGateway {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<user::Model, String> {
        let user: Option<user::Model> = user::Entity::find_by_id(user_id).one(&self.db).await.unwrap();
        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    async fn get_user_by_username(&self, username: String) -> Result<user::Model, String> {
        let user: Option<user::Model> = user::Entity::find().filter(
                user::Column::Username.eq(username)
            )
            .one(&self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<user::Model, String> {
        let user: Option<user::Model> = user::Entity::find().filter(
                user::Column::Email.eq(email)
            )
            .one(&self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }
}