use deadpool_redis::Pool;
use sea_orm::DbConn;
use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::sha256_session_hasher::Sha256SessionHasher;

use crate::adapters::database::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;
use crate::adapters::database::role_db::RoleGateway;
use crate::application::common::id_provider::IdProvider;
use crate::application::session::create::CreateSession;
use crate::application::session::delete::DeleteSession;
use crate::application::session::delete_self::DeleteSessionSelf;
use crate::application::session::extract_payload::EPSession;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionsByUserId;
use crate::application::session::get_self::GetSessionSelf;
use crate::application::user::create::CreateUser;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::user::get_self::GetUserSelf;
use crate::application::user::update::UpdateUser;
use crate::application::user::update_self::UpdateUserSelf;
use crate::domain::services::access::AccessService;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    session_gateway: SessionGateway,
    role_gateway: RoleGateway,
    user_service: UserService,
    session_service: SessionService,
    password_hasher: Argon2PasswordHasher,
    session_hasher: Sha256SessionHasher,
    validator: ValidatorService,
    access_service: AccessService,
}

impl IoC {
    pub fn new(
        db_pool: Box<DbConn>,
        session_redis_pool: Pool,
        confirm_manager_redis_pool: Pool,
    ) -> IoC {
        IoC {
            user_gateway: UserGateway::new(db_pool.clone()),
            session_gateway: SessionGateway::new(
                Box::new(session_redis_pool),
                db_pool.clone(),
            ),
            role_gateway: RoleGateway::new(db_pool.clone()),
            user_service: UserService{},
            session_service: SessionService{},
            password_hasher: Argon2PasswordHasher::new(),
            session_hasher: Sha256SessionHasher {},
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
            password_hasher: &self.password_hasher,
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

    fn update_user_self(&self, id_provider: Box<dyn IdProvider>) -> UpdateUserSelf {
        UpdateUserSelf {
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            validator: &self.validator,
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
            password_hasher: &self.password_hasher,
            session_hasher: &self.session_hasher,
            validator: &self.validator,
            access_service: &self.access_service,
            role_reader: &self.role_gateway,
        }
    }

    fn delete_session(&self, id_provider: Box<dyn IdProvider>) -> DeleteSession {
        DeleteSession {
            session_gateway: &self.session_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn delete_self_session(&self, id_provider: Box<dyn IdProvider>) -> DeleteSessionSelf {
        DeleteSessionSelf {
            session_remover: &self.session_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }

    fn get_session_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionById {
        GetSessionById {
            session_reader: &self.session_gateway,
            access_service: &self.access_service,
            id_provider,
        }
    }

    fn get_sessions_by_user_id(&self, id_provider: Box<dyn IdProvider>) -> GetSessionsByUserId {
        GetSessionsByUserId {
            session_reader: &self.session_gateway,
            access_service: &self.access_service,
            id_provider,
        }
    }

    fn get_sessions_self(&self, id_provider: Box<dyn IdProvider>) -> GetSessionSelf {
        GetSessionSelf {
            session_reader: &self.session_gateway,
            access_service: &self.access_service,
            id_provider,
        }
    }
    
    fn extract_payload(&self, id_provider: Box<dyn IdProvider>) -> EPSession {
        EPSession {
            session_gateway: &self.session_gateway,
            user_gateway: &self.user_gateway,
            session_service: &self.session_service,
            session_hasher: &self.session_hasher,
            id_provider,
            validator_service: &self.validator,
        }
    }
    
}