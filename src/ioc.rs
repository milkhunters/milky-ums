use crate::adapters::database::connector::DbConnector;
use crate::adapters::redis::session_db::SessionGateway;
use crate::adapters::database::user_db::UserGateway;
use crate::adapters::redis::pool::RedisAsyncPool;
use crate::application::user::get_by_id::GetUserById;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    session_gateway: SessionGateway,
}

impl IoC {
    pub fn new(db: DbConnector, redis_pool: RedisAsyncPool) -> Self {
        IoC {
            user_gateway: UserGateway::new(db.clone()),
            session_gateway: SessionGateway::new(redis_pool)
        }
    }
}

impl InteractorFactory for IoC {
    fn get_user_by_id(&self) -> GetUserById {
        GetUserById {
            user_gateway: &self.user_gateway,
        }
    }
}