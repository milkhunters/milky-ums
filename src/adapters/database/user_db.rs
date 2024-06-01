use core::option::Option;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use cached::{Cached, TimedCache};
use sea_orm::{DbConn, EntityTrait, QueryFilter, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Expr};
use sea_orm::sea_query::extension::postgres::PgExpr;
use uuid::Uuid;

use crate::adapters::database::models::user;
use crate::application::common::user_gateway::{
    UserReader, 
    UserWriter, 
    UserGateway as UserGatewayTrait
};
use crate::domain::models::user::User as UserDomain;
use crate::domain::models::user::UserState as UserStateDomain;

pub struct UserGateway{
    pub db: Box<DbConn>,
    cache_user_by_id: Arc<Mutex<TimedCache<Uuid, user::Model>>>,
}

impl UserGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        UserGateway {
            db,
            cache_user_by_id: Arc::new(Mutex::new(TimedCache::with_lifespan(3))),

        }
    }
}

#[async_trait]
impl UserReader for UserGateway {
    async fn get_user_by_id(&self, user_id: &Uuid) -> Option<UserDomain> {

        let cached_value = self.cache_user_by_id.lock().unwrap().cache_get(user_id).cloned();
        if cached_value.is_some() {
            return match cached_value {
                Some(value) => Some(map_user_model_to_domain(value.clone())),
                None => None
            }
        }

        match user::Entity::find_by_id(user_id.clone()).one(&*self.db).await.unwrap() {
            Some(user) => {
                self.cache_user_by_id.lock().unwrap().cache_set(user_id.clone(), user.clone());
                Option::from(map_user_model_to_domain(user))
            }
            None => None
        }
    }

    async fn get_users_by_ids(&self, user_ids: &Vec<Uuid>) -> Option<Vec<UserDomain>> {
        let users: Vec<user::Model> = user::Entity::find().filter(
            {
                let mut condition = Condition::any();
                for id in user_ids {
                    condition = condition.add(Expr::col(user::Column::Id).eq(*id));
                }
                condition
            }
            )
            .all(&*self.db)
            .await
            .unwrap();

        if users.len() != user_ids.len() {
            return None
        }

        Option::from(
            users.iter()
                .map(|user| map_user_model_to_domain(user.clone()))
                .collect::<Vec<_>>()
        )
    }

    async fn get_users_list(&self, limit: &u64, offset: &u64) -> Vec<UserDomain> {
        let users: Vec<user::Model> = user::Entity::find()
            .limit(limit.clone())
            .offset(offset.clone())
            .all(&*self.db)
            .await
            .unwrap();
        users.iter().map(|user| map_user_model_to_domain(user.clone())).collect()
    }

    async fn get_user_by_username_not_sensitive(&self, username: &String) -> Option<UserDomain> {
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

    async fn get_user_by_email_not_sensitive(&self, email: &String) -> Option<UserDomain> {
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

#[async_trait]
impl UserWriter for UserGateway {
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

        match user::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                user::Entity::update(user_model).exec(&*self.db).await.unwrap();
            }
            None => {
                user::Entity::insert(user_model).exec(&*self.db).await.unwrap();
            }
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

impl UserGatewayTrait for UserGateway {}