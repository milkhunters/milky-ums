use async_trait::async_trait;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use sea_orm::sea_query::{Condition, Expr};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user;

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
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<user::Model, ApplicationError> {
        match user::Entity::find_by_id(user_id).one(&*self.db).await.unwrap() {
            Some(user) => Ok(user),
            None => Err(ApplicationError::NotFound(ErrorContent::Message("Пользователь не найден".to_string())))
        }
    }

    async fn get_users_by_ids(&self, user_ids: Vec<Uuid>) -> Result<Vec<user::Model>, ApplicationError> {
        let users: Vec<user::Model> = user::Entity::find().filter(
            {
                let mut condition = Condition::any();
                for id in &user_ids {
                    condition = condition.add(Expr::col(user::Column::Id).eq(*id));
                }
                condition
            }
            )
            .all(&*self.db)
            .await
            .unwrap();

        if users.len() != *&user_ids.len() {
            return Err(ApplicationError::NotFound(ErrorContent::Message(
                "Не все запрашиваемые пользователи найдены".to_string()
            )));
        }

        Ok(users)
    }

    async fn get_user_by_username(&self, username: String) -> Result<user::Model, ApplicationError> {
        let user: Option<user::Model> = user::Entity::find().filter(
                user::Column::Username.eq(username)
            )
            .one(&*self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err(ApplicationError::NotFound(ErrorContent::Message("User not found".to_string())))
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<user::Model, ApplicationError> {
        let user: Option<user::Model> = user::Entity::find().filter(
                user::Column::Email.eq(email)
            )
            .one(&*self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Ok(user),
            None => Err(ApplicationError::NotFound(ErrorContent::Message("User not found".to_string())))
        }
    }

    async fn get_list(&self) -> Result<Vec<user::Model>, ApplicationError> {
        let users: Vec<user::Model> = user::Entity::find().all(&*self.db).await.unwrap();
        Ok(users)
    }
}