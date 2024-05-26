use deadpool_redis::Pool;
use sea_orm::DbConn;

use crate::adapters::argon2_hasher::Argon2Hasher;
use crate::adapters::database::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;
use crate::application::common::id_provider::IdProvider;
use crate::application::session::create::CreateSession;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionByUserId;
use crate::application::user::create::CreateUser;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::user::get_self::GetUserSelf;
use crate::application::user::update_by_id::UpdateUser;
use crate::domain::services::access::AccessService;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    session_gateway: SessionGateway,
    user_service: UserService,
    session_service: SessionService,
    hasher: Argon2Hasher,
    validator: ValidatorService,
    access_service: AccessService,
}

impl IoC {
    pub fn new(
        db: DbConn,
        session_redis_pool: Pool,
        confirm_manager_redis_pool: Pool,
    ) -> IoC {

        let db_pool = Box::new(db);

        IoC {
            user_gateway: UserGateway::new(db_pool.clone()),
            session_gateway: SessionGateway::new(Box::new(session_redis_pool)),
            user_service: UserService{},
            session_service: SessionService{},
            hasher: Argon2Hasher::new(),
            validator: ValidatorService::new(),
            access_service: AccessService{},
        }
    }
}

impl InteractorFactory for IoC {
    fn get_user_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetUserById {
        GetUserById {
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_users_by_ids(&self, id_provider: Box<dyn IdProvider>) -> GetUsersByIds {
        GetUsersByIds {
            id_provider,
            user_reader: &self.user_gateway,
            access_service: &self.access_service,
        }
    }

    fn get_user_range(&self, id_provider: Box<dyn IdProvider>) -> GetUserRange {
        GetUserRange {
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }
    
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf {
        GetUserSelf {
            user_reader: &self.user_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser {
        CreateUser {
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            password_hasher: &self.hasher,
            validator: &self.validator,
            access_service: &self.access_service,
            id_provider,
        }
    }
    
    fn update_user(&self, id_provider: Box<dyn IdProvider>) -> UpdateUser {
        UpdateUser {
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            validator: &self.validator,
            access_service: &self.access_service,
            id_provider,
        }
    }

    fn get_session_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionById {
        GetSessionById {
            session_reader: &self.session_gateway,
            access_service: &self.access_service,
            id_provider,
        }
    }

    fn get_sessions_by_user_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionByUserId {
        GetSessionByUserId {
            session_reader: &self.session_gateway,
            access_service: &self.access_service,
            id_provider,
        }
    }

    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession {
        CreateSession {
            id_provider,
            session_gateway: &self.session_gateway,
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            session_service: &self.session_service,
            password_hasher: &self.hasher,
            validator: &self.validator,
            access_service: &self.access_service,
        }
    }
}