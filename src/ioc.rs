use deadpool_redis::Pool;
use sea_orm::DbConn;

use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::database::access_log_db::AccessLogGateway;
use crate::adapters::database::permission_db::PermissionGateway;
use crate::adapters::database::role_db::RoleGateway;
use crate::adapters::database::service_db::ServiceGateway;
use crate::adapters::database::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;
use crate::adapters::redis_confirm_code::RedisConfirmCode;
use crate::adapters::rmq_email_sender::RMQEmailSender;
use crate::adapters::sha256_session_hasher::Sha256SessionHasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::service::sync::ServiceSync;
use crate::application::session::create::CreateSession;
use crate::application::session::delete::DeleteSession;
use crate::application::session::delete_self::DeleteSessionSelf;
use crate::application::session::extract_payload::EPSession;
use crate::application::session::get_access_log::GetAccessLog;
use crate::application::session::get_access_log_self::GetAccessLogSelf;
use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionsByUserId;
use crate::application::session::get_self::GetSessionSelf;
use crate::application::user::change_password::ChangePassword;
use crate::application::user::confirm::ConfirmUser;
use crate::application::user::create::CreateUser;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::application::user::get_self::GetUserSelf;
use crate::application::user::reset_password::ResetPassword;
use crate::application::user::send_confirm_code::SendConfirmCode;
use crate::application::user::update::UpdateUser;
use crate::application::user::update_self::UpdateUserSelf;
use crate::config::Extra;
use crate::domain::services::access::AccessService;
use crate::domain::services::access_log::AccessLogService;
use crate::domain::services::session::SessionService;
use crate::domain::services::user::UserService;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    session_gateway: SessionGateway,
    access_log_gateway: AccessLogGateway,
    access_log_service: AccessLogService,
    role_gateway: RoleGateway,
    service_gateway: ServiceGateway,
    permission_gateway: PermissionGateway,
    user_service: UserService,
    session_service: SessionService,
    password_hasher: Argon2PasswordHasher,
    session_hasher: Sha256SessionHasher,
    validator: ValidatorService,
    access_service: AccessService,
    confirm_code: RedisConfirmCode,
    email_sender: RMQEmailSender,
    extra: Extra,
}

impl IoC {
    pub fn new(
        db_pool: Box<DbConn>,
        session_redis_pool: Pool,
        session_exp: u32,
        email_sender: RMQEmailSender,
        confirm_redis_pool: Pool,
        confirm_code_ttl: u32,
        extra: Extra
    ) -> IoC {
        IoC {
            user_gateway: UserGateway::new(db_pool.clone()),
            session_gateway: SessionGateway::new(
                Box::new(session_redis_pool),
                session_exp,
                db_pool.clone(),
            ),
            access_log_gateway: AccessLogGateway::new(db_pool.clone()),
            access_log_service: AccessLogService {},
            role_gateway: RoleGateway::new(db_pool.clone()),
            service_gateway: ServiceGateway::new(db_pool.clone()),
            permission_gateway: PermissionGateway::new(db_pool.clone()),
            user_service: UserService{},
            session_service: SessionService::new(session_exp),
            password_hasher: Argon2PasswordHasher::new(),
            session_hasher: Sha256SessionHasher {},
            validator: ValidatorService::new(),
            access_service: AccessService{},
            confirm_code: RedisConfirmCode::new(
                Box::new(confirm_redis_pool),
                confirm_code_ttl,
            ),
            email_sender,
            extra
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
            role_gateway: &self.role_gateway,
            email_sender: &self.email_sender,
            user_service: &self.user_service,
            password_hasher: &self.password_hasher,
            validator: &self.validator,
            access_service: &self.access_service,
            id_provider,
            extra: &self.extra,
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
            access_log_writer: &self.access_log_gateway,
            access_log_service: &self.access_log_service,
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
            session_service: &self.session_service,
            session_hasher: &self.session_hasher,
            id_provider,
            validator_service: &self.validator,
        }
    }

    fn sync_service(&self) -> ServiceSync {
        ServiceSync {
            service_gateway: &self.service_gateway,
            permission_gateway: &self.permission_gateway,
        }
    }

    fn send_confirm_code(&self, id_provider: Box<dyn IdProvider>) -> SendConfirmCode {
        SendConfirmCode {
            id_provider,
            email_sender: &self.email_sender,
            confirm_code: &self.confirm_code,
            user_reader: &self.user_gateway,
            access_service: &self.access_service,
            validator: &self.validator,
        }
    }

    fn confirm_user(&self, id_provider: Box<dyn IdProvider>) -> ConfirmUser {
        ConfirmUser {
            id_provider,
            user_gateway: &self.user_gateway,
            email_sender: &self.email_sender,
            extra: &self.extra,
            confirm_code: &self.confirm_code,
            user_service: &self.user_service,
            validator: &self.validator,
            access_service: &self.access_service,
        }
    }
    
    fn change_password(&self, id_provider: Box<dyn IdProvider>) -> ChangePassword {
        ChangePassword {
            email_sender: &self.email_sender,
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            validator: &self.validator,
            password_hasher: &self.password_hasher,
            access_service: &self.access_service,
            id_provider,
            extra: &self.extra,
        }
    }

    fn reset_password(&self, id_provider: Box<dyn IdProvider>) -> ResetPassword {
        ResetPassword {
            email_sender: &self.email_sender,
            confirm_code: &self.confirm_code,
            user_gateway: &self.user_gateway,
            user_service: &self.user_service,
            validator: &self.validator,
            password_hasher: &self.password_hasher,
            access_service: &self.access_service,
            session_remover: &self.session_gateway,
            id_provider,
            extra: &self.extra,
        }
    }
    
    fn get_access_log_self(&self, id_provider: Box<dyn IdProvider>) -> GetAccessLogSelf {
        GetAccessLogSelf {
            access_log_reader: &self.access_log_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }
    
    fn get_access_log(&self, id_provider: Box<dyn IdProvider>) -> GetAccessLog {
        GetAccessLog {
            access_log_reader: &self.access_log_gateway,
            id_provider,
            access_service: &self.access_service,
        }
    }
}