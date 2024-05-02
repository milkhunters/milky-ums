use deadpool_redis::Pool;
use sea_orm::DbConn;

use crate::adapters::argon2_hasher::Argon2Hasher;
use crate::adapters::database::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionByUserId;
use crate::application::user::create_user::CreateUser;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    session_gateway: SessionGateway,
    user_service: UserService,
    hasher: Argon2Hasher,
    validator: ValidatorService
}

impl IoC {
    pub fn new(
        db: DbConn,
        session_redis_pool: Pool,
        confirm_manager_redis_pool: Pool
    ) -> IoC {

        let db_pool = Box::new(db);

        IoC {
            user_gateway: UserGateway::new(db_pool.clone()),
            session_gateway: SessionGateway::new(Box::new(session_redis_pool)),
            user_service: UserService{},
            hasher: Argon2Hasher::new(),
            validator: ValidatorService::new()
        }
    }
}

impl InteractorFactory for IoC {
    fn get_user_by_id(&self) -> GetUserById {
        GetUserById {
            user_gateway: &self.user_gateway
        }
    }

    fn get_users_by_ids(&self) -> GetUsersByIds {
        GetUsersByIds {
            user_gateway: &self.user_gateway
        }
    }

    fn get_user_range(&self) -> GetUserRange {
        GetUserRange {
            user_gateway: &self.user_gateway
        }
    }

    fn create_user(&self) -> CreateUser {
        CreateUser {
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            password_hasher: &self.hasher,
            validator: &self.validator
        }
    }

    fn get_session_by_id(&self) -> GetSessionById {
        GetSessionById {
            session_gateway: &self.session_gateway,
        }
    }

    fn get_sessions_by_user_id(&self) -> GetSessionByUserId {
        GetSessionByUserId {
            session_gateway: &self.session_gateway,
        }
    }
}