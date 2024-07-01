use std::thread;

use deadpool_redis::{Config, Runtime};
use dotenv::dotenv;
use lapin::{Connection as RabbitConnection, ConnectionProperties as RMQConnProps};
use sea_orm::{ConnectOptions, Database, DbConn};
use tera::Tera;

use crate::application::common::server::Server;
use crate::domain::models::service::ServiceTextId;
use crate::ioc::IoC;
use crate::presentation::grpc::server::GrpcServer;
use crate::presentation::web::server::{AppConfigProvider, HttpServer};

mod config;
mod presentation;
mod application;
mod adapters;
mod domain;
mod ioc;


fn main() -> std::io::Result<()> {
    dotenv().ok();

    if let Some(log_level) = &std::env::var("LOG_LEVEL").ok() {
        std::env::set_var("RUST_LOG", log_level);
    }


    env_logger::builder()
        .filter_module("consulrs", log::LevelFilter::Error)
        .filter_module("tracing", log::LevelFilter::Error)
        .filter_module("rustify", log::LevelFilter::Error)
        .init();

    let http_workers = match std::env::var("HTTP_WORKERS") {
        Ok(workers) => workers.parse::<usize>().ok(),
        Err(_) => None,
    };

    let grpc_workers = match std::env::var("GRPC_WORKERS") {
        Ok(workers) => workers.parse::<usize>().ok(),
        Err(_) => None,
    };
    
    let http_host = std::env::var("HTTP_HOST").unwrap_or("127.0.0.1".to_string());
    let http_port = std::env::var("HTTP_PORT").unwrap_or("8080".to_string());
    let grpc_host = std::env::var("GRPC_HOST").unwrap_or("127.0.0.1".to_string());
    let grpc_port = std::env::var("GRPC_PORT").unwrap_or("50051".to_string());
    let is_intermediate: bool = std::env::var("IS_INTERMEDIATE").unwrap_or("false".to_string()).parse().unwrap();
    let service_name: ServiceTextId = std::env::var("SERVICE_NAME").unwrap();
    let consul_addr = std::env::var("CONSUL_ADDR").unwrap();
    let consul_root = std::env::var("CONSUL_ROOT").unwrap();
    let build = std::env::var("BUILD").unwrap_or("local".to_string());
    let branch = std::env::var("BRANCH").unwrap_or("unknown".to_string());
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let config = match rt.block_on(config::Config::from_consul(&consul_addr, &consul_root)) {
        Ok(config) => config,
        Err(error) => {
            log::error!("Failed to load config: {}", error);
            std::process::exit(1);
        },
    };

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Tera parsing error(s): {}", e);
            std::process::exit(1);
        }
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
        rt.block_on(Database::connect(opt))
    } {
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
    let confirm_code_redis_pool = redis_factory(1).create_pool(Some(Runtime::Tokio1)).unwrap();

    rt.block_on(application::initial::service_permissions(
        &adapters::database::service_db::ServiceGateway::new(db.clone()),
        &adapters::database::permission_db::PermissionGateway::new(db.clone()),
        &domain::services::permission::PermissionService{},
        service_name.clone(),
        &domain::services::external::ExternalService{},
    ));

    rt.block_on(application::initial::control_account(
        &adapters::database::role_db::RoleGateway::new(db.clone()),
        &domain::services::role::RoleService{},
        &adapters::database::permission_db::PermissionGateway::new(db.clone()),
        &adapters::database::user_db::UserGateway::new(db.clone()),
        &domain::services::user::UserService{},
        &adapters::argon2_password_hasher::Argon2PasswordHasher::new(),
        &adapters::database::init_state_db::InitStateGateway::new(db.clone()),
    ));
    
    let ioc_factory = || {
        IoC::new(
            db.clone(),
            session_redis_pool.clone(),
            config.base.session_exp,
            rt.block_on(async {
                let rmq_conn = RabbitConnection::connect(
                    &format!("amqp://{username}:{password}@{host}:{port}/{vhost}",
                             username = config.email.rabbitmq.username,
                             password = config.email.rabbitmq.password,
                             host = config.email.rabbitmq.host,
                             port = config.email.rabbitmq.port,
                             vhost = config.email.rabbitmq.vhost,
                    ),
                    RMQConnProps::default(),
                ).await.map_err(|error| {
                    log::error!("Failed to connect to RabbitMQ: {}", error);
                    std::process::exit(1);
                }).unwrap();

                adapters::rmq_email_sender::RMQEmailSender::new(
                    Box::new(rmq_conn),
                    config.email.rabbitmq.exchange.clone(),
                    config.email.sender_id.clone(),
                    service_name.clone(),
                    tera.clone(),
                )
            }),
            confirm_code_redis_pool.clone(),
            config.base.confirm_code_ttl,
            config.base.extra.clone()
        )
    };
    
    let app_config_provider = AppConfigProvider {
        branch,
        build,
        service_name: service_name.clone(),
        version: VERSION,
        is_intermediate,
    };

    let http_server = HttpServer::new(
        app_config_provider,
        ioc_factory()
    )
        .bind(format!("{}:{}", http_host, http_port)).map_err(|error| {
            log::error!("Http Server: failed to bind to port {} in host {}: {}", http_host, http_port, error);
            std::process::exit(1);
        }).unwrap()
        .set_workers(
            http_workers.unwrap_or_else(|| {
                match thread::available_parallelism() {
                    Ok(parallelism) => {
                        let available_workers = usize::from(parallelism);
                        if available_workers > 1 {
                            available_workers / 2
                        } else {
                            available_workers
                        }
                    }
                    Err(_) => 1,
                }
            })
        );
    
    
    let grpc_server_addr = format!("{}:{}", grpc_host, grpc_port);
    
    let grpc_server = GrpcServer::new(
        service_name.clone(),
        ioc_factory()
    )
        .bind(grpc_server_addr.clone()).map_err(|error| {
            log::error!("gRPC Server: failed to bind to address {}: {}", grpc_server_addr, error);
            std::process::exit(1);
        }).unwrap()
        .set_workers(
            grpc_workers.unwrap_or_else(|| {
                match thread::available_parallelism() {
                    Ok(parallelism) => {
                        let available_workers = usize::from(parallelism);
                        if available_workers > 1 {
                            available_workers / 2
                        } else {
                            available_workers
                        }
                    },
                    Err(_) => 1,
                }
            })
        );
    
    thread::Builder::new()
        .name("gRPC Server".into())
        .spawn(move || {
            grpc_server.run().unwrap();
        }).unwrap();

    http_server.run().unwrap();
    Ok(())
}
