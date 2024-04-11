use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use dotenv::dotenv;
use actix_web::{App, HttpServer, web};
use sea_orm::{ConnectOptions, Database};
use deadpool_redis::{redis::{cmd, FromRedisValue}, Config, Runtime};
use redis::Client;
use crate::adapters::database::initializer::create_tables;

use crate::adapters::database::user_db::UserGateway;
use crate::ioc::IoC;


mod config;
mod presentation;
mod application;
mod adapters;
mod domain;
mod ioc;

struct AppConfigProvider {
    branch: String,
    build: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    let workers = match std::env::var("WORKERS") {
        Ok(workers) => workers.parse::<usize>().ok(),
        Err(_) => None,
    };

    let consul_addr = std::env::var("CONSUL_ADDR").unwrap();
    let consul_root = std::env::var("CONSUL_ROOT").unwrap();
    let build = std::env::var("BUILD").unwrap_or("local".to_string());
    let branch = std::env::var("BRANCH").unwrap_or("unknown".to_string());


    let config = match config::Config::from_consul(&consul_addr, &consul_root).await {
        Ok(config) => config,
        Err(error) => {
            log::error!("Failed to load config: {}", error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
        },
    };

    let db = match {
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
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);
        Database::connect(opt)
    }.await {
        Ok(db) => db,
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    create_tables(&db).await;

    let redis: Config = Config::from_url(format!(
        "redis://{username}:{password}@{host}:{port}/{db}",
        username = config.database.redis.username.unwrap_or("default".to_string()),
        password = config.database.redis.password,
        host = config.database.redis.host,
        port = config.database.redis.port,
        db = "0"
    ));

    let redis_pool = redis.create_pool(Some(Runtime::Tokio1)).unwrap();

    let app_builder = move || {
        App::new()
            .configure(presentation::api::user::router)
            .app_data(web::Data::new(AppConfigProvider {
                branch: branch.clone(),
                build: build.clone(),
            }))
            .app_data(web::Data::new(
                IoC::new(
                    db.clone(),
                    redis_pool.clone(),
                )
            ))
        // .wrap(Logger::new("[%s] [%{r}a] %U"))
    };


    let available_workers = workers.unwrap_or(
        match thread::available_parallelism() {
            Ok(parallelism) => usize::from(parallelism),
            Err(_) => 1,
        }
    );

    let host = "127.0.0.1";
    let port = 8080;

    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(listener) => {
            log::info!("🚀 Server started at http://{}:{}", host, port);
            listener
        },
        Err(e) => {
            log::error!("Failed to bind to port {} in host {}: {}", port, host, e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        },
    };

    HttpServer::new(app_builder)
        .listen(listener)?
        .workers(available_workers)
        .run().await
}
