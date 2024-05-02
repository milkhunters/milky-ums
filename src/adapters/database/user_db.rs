use async_trait::async_trait;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Expr};
use core::option::Option;
use sea_orm::sea_query::extension::postgres::PgExpr;
use uuid::Uuid;

use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::User as UserDomain;
use crate::domain::models::user::UserState as UserStateDomain;
use crate::adapters::database::models::user;

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
    async fn get_user_by_id(&self, user_id: Uuid) -> Option<UserDomain> {
        match user::Entity::find_by_id(user_id).one(&*self.db).await.unwrap() {
            Some(user) => Option::from(map_user_model_to_domain(user)),
            None => None
        }
    }

    async fn get_users_by_ids(&self, user_ids: Vec<Uuid>) -> Option<Vec<UserDomain>> {
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
            return None
        }

        Option::from(
            users.iter()
                .map(|user| map_user_model_to_domain(user.clone()))
                .collect::<Vec<_>>()
        )
    }

    async fn get_list(&self, limit: u64, offset: u64) -> Vec<UserDomain> {
        let users: Vec<user::Model> = user::Entity::find()
            .limit(limit)
            .offset(offset)
            .all(&*self.db)
            .await
            .unwrap();
        users.iter().map(|user| map_user_model_to_domain(user.clone())).collect()
    }

    async fn save_user(&self, data: &UserDomain) {
        let user_model = user::ActiveModel {
            id: Set(data.id),
            username: Set(data.username.clone()),
            email: Set(data.email.clone()),
            first_name: Set(data.first_name.clone()),
            last_name: Set(data.last_name.clone()),
            state: Set(match data.state {
                UserStateDomain::Active => user::UserState::Active,
                UserStateDomain::Inactive => user::UserState::Inactive,
                UserStateDomain::Banned => user::UserState::Banned,
                UserStateDomain::Deleted => user::UserState::Deleted
            }),
            hashed_password: Set(data.hashed_password.clone()),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at.clone())
        };

        user::Entity::insert(user_model).exec(&*self.db).await.unwrap();
    }

    async fn get_user_by_username_not_sensitive(&self, username: String) -> Option<UserDomain> {
        let user: Option<user::Model> = user::Entity::find().filter(
                Expr::col(user::Column::Username).ilike(username)
            )
            .one(&*self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Some(map_user_model_to_domain(user)),
            None => None
        }
    }

    async fn get_user_by_email_not_sensitive(&self, email: String) -> Option<UserDomain> {
        let user: Option<user::Model> = user::Entity::find().filter(
                Expr::col(user::Column::Email).ilike(email)
            )
            .one(&*self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Some(map_user_model_to_domain(user)),
            None => None
        }
    }
}


fn map_user_model_to_domain(user: user::Model) -> UserDomain {
    UserDomain {
        id: user.id,
        username: user.username,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        state: match user.state {
            user::UserState::Active => UserStateDomain::Active,
            user::UserState::Inactive => UserStateDomain::Inactive,
            user::UserState::Banned => UserStateDomain::Banned,
            user::UserState::Deleted => UserStateDomain::Deleted
        },
        hashed_password: user.hashed_password,
        created_at: user.created_at,
        updated_at: user.updated_at
    }
}
