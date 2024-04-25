use async_trait::async_trait;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user;
use crate::domain::models::user::Model;

pub struct UserGateway{
    pub db: Box<DbConn>
}

impl UserGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        UserGateway {
            db
        }
    }
}
#[async_trait]
impl UserReader for UserGateway {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<user::Model, String> {
        match user::Entity::find_by_id(user_id).one(&*self.db).await.unwrap() {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    async fn get_user_by_username(&self, username: String) -> Result<user::Model, String> {
        let user: Option<user::Model> = user::Entity::find().filter(
                user::Column::Username.eq(username)
            )
            .one(&*self.db)
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
            .one(&*self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    async fn get_list(&self) -> Result<Vec<Model>, String> {
        let users: Vec<Model> = user::Entity::find().all(&*self.db).await.unwrap();
        Ok(users)
    }
}