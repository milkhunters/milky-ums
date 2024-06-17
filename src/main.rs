use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use dotenv::dotenv;
use actix_web::{App, HttpServer, web};
use sea_orm::{ConnectOptions, Database, DbConn};
use deadpool_redis::{Config, Runtime};
use crate::domain::models::service::ServiceTextId;

use crate::ioc::IoC;
use crate::presentation::interactor_factory::InteractorFactory;


mod config;
mod presentation;
mod application;
mod adapters;
mod domain;
mod ioc;

struct AppConfigProvider {
    branch: String,
    build: String,
    service_name: ServiceTextId,
    version: &'static str,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    env_logger::builder()
        .filter_module("consulrs", log::LevelFilter::Error)
        .filter_module("tracing", log::LevelFilter::Error)
        .filter_module("rustify", log::LevelFilter::Error)
        .init();


    let workers = match std::env::var("WORKERS") {
        Ok(workers) => workers.parse::<usize>().ok(),
        Err(_) => None,
    };
    
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let service_name: ServiceTextId = std::env::var("SERVICE_NAME").unwrap();
    let consul_addr = std::env::var("CONSUL_ADDR").unwrap();
    let consul_root = std::env::var("CONSUL_ROOT").unwrap();
    let build = std::env::var("BUILD").unwrap_or("local".to_string());
    let branch = std::env::var("BRANCH").unwrap_or("unknown".to_string());
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let config = match config::Config::from_consul(&consul_addr, &consul_root).await {
        Ok(config) => config,
        Err(error) => {
            log::error!("Failed to load config: {}", error);
            std::process::exit(1);
        },
    };

    let db: Box<DbConn> = match {
        let mut opt = ConnectOptions::new(
            format!(
                "postgresql://{username}:{password}@{host}:{port}/{database}",
                username = config.database.postgresql.username,
                password = config.database.postgresql.password,
                host = config.database.postgresql.host,
                port = config.database.postgresql.port,
                database = config.database.postgresql.database,
            )
        );
        opt.max_connections(40)
            .min_connections(5)
            .sqlx_logging(false);
        Database::connect(opt)
    }.await {
        Ok(db) => Box::new(db),
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        },
    };

    let redis_factory = move |db| {
        if !(0 <= db && db <= 15) {
            log::error!("Invalid Redis database number: {}", db);
            std::process::exit(1);
        }
        let cfg = config.database.redis.clone();
        Config::from_url(format!(
            "redis://{username}:{password}@{host}:{port}/{db}",
            username = cfg.username.unwrap_or_else(|| "default".to_string()),
            password = cfg.password,
            host = cfg.host,
            port = cfg.port,
            db = db,
        ))
    };
    
    let session_redis_pool = redis_factory(0).create_pool(Some(Runtime::Tokio1)).unwrap();
    let confirm_manager_redis_pool = redis_factory(1).create_pool(Some(Runtime::Tokio1)).unwrap();
    
    application::initial::service_permissions(
        &adapters::database::service_db::ServiceGateway::new(db.clone()),
        &adapters::database::permission_db::PermissionGateway::new(db.clone()),
        &service_name,
    ).await;
    
    application::initial::control_account(
        &adapters::database::role_db::RoleGateway::new(db.clone()),
        &domain::services::role::RoleService{},
        &adapters::database::permission_db::PermissionGateway::new(db.clone()),
        &adapters::database::user_db::UserGateway::new(db.clone()),
        &domain::services::user::UserService{},
        &adapters::argon2_password_hasher::Argon2PasswordHasher::new(),
        &adapters::database::init_state_db::InitStateGateway::new(db.clone()),
    ).await;
    
    let app_builder = move || {
        let branch = branch.clone();
        let build = build.clone();
        
        let ioc_arc: Arc<dyn InteractorFactory> = Arc::new(IoC::new(
            db.clone(),
            session_redis_pool.clone(),
            confirm_manager_redis_pool.clone(),
        ));
        let ioc_data: web::Data<dyn InteractorFactory> = web::Data::from(ioc_arc);
        
        App::new()
            .service(web::scope("/api")
                .configure(presentation::web::rest::user::router)
                .configure(presentation::web::rest::session::router)
            )
            .app_data(web::Data::new(AppConfigProvider {
                branch,
                build,
                service_name: service_name.clone(),
                version: VERSION,
            }))
            .app_data(ioc_data)
            .default_service(web::route().to(presentation::web::exception::not_found))
        // .wrap(Logger::new("[%s] [%{r}a] %U"))
    };


    let available_workers = workers.unwrap_or(
        match thread::available_parallelism() {
            Ok(parallelism) => usize::from(parallelism),
            Err(_) => 1,
        }
    );
    
    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(listener) => {
            listener
        },
        Err(e) => {
            log::error!("Failed to bind to port {} in host {}: {}", port, host, e);
            std::process::exit(1);
        },
    };

    let server = HttpServer::new(app_builder).listen(listener)?.workers(available_workers);

    server.addrs_with_scheme().iter().for_each(|addr| {
        let (socket_addr, str_ref) = addr;
        log::info!("🚀 Server started at {}://{:?}", str_ref, socket_addr);
    });
    
    server.run().await.and_then(|_| {
        log::info!("Server stopped!");
        Ok(())
    })
}
