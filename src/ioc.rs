use deadpool_redis::Pool;
use sea_orm::DbConn;

use crate::adapters::database::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;

use crate::application::session::get_by_id::GetSessionById;
use crate::application::session::get_by_user_id::GetSessionByUserId;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_by_ids::GetUsersByIds;
use crate::application::user::get_range::GetUserRange;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    session_gateway: SessionGateway
}

impl IoC {
    pub fn new(
        db: DbConn,
        session_redis_pool: Pool,
        confirm_manager_redis_pool: Pool
    ) -> Self {

        let db_pool = Box::new(db);

        IoC {
            user_gateway: UserGateway::new(db_pool.clone()),
            session_gateway: SessionGateway::new(Box::new(session_redis_pool))
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