use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{App, HttpServer as ActixHttpServer, web};
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::application::common::server::{ConnectionConfig, Server};
use crate::domain::models::service::ServiceTextId;
use crate::ioc::IoC;
use crate::presentation;
use crate::presentation::interactor_factory::InteractorFactory;

#[derive(Clone)]
pub struct AppConfigProvider {
    pub branch: String,
    pub build: String,
    pub service_name: ServiceTextId,
    pub version: &'static str,
    pub is_intermediate: bool,
}


pub struct HttpServer {
    connection_config: Arc<Mutex<ConnectionConfig>>,
    app_config_provider: AppConfigProvider,
    ioc: Arc<dyn InteractorFactory>
}

impl HttpServer {
    pub fn new(
        app_config_provider: AppConfigProvider,
        ioc: IoC
    ) -> Self {
        
        let ioc_arc: Arc<dyn InteractorFactory> = Arc::new(ioc);
        
        Self {
            connection_config: Arc::new(Mutex::new(ConnectionConfig::default())),
            app_config_provider,
            ioc: ioc_arc,
        }
    }
}

impl Server for HttpServer {
    fn bind(self, addr: String) -> io::Result<Self> {
        self.connection_config.lock().unwrap().tcp_listener = Some(
            TcpListener::bind(addr)?
        );
        Ok(self)
    }

    fn set_workers(self, workers: usize) -> Self {
        self.connection_config.lock().unwrap().workers = workers;
        self
    }

    fn set_tls(self, key: &str, cert: &str) -> Self {
        self.connection_config.lock().unwrap().tls = Some((key.to_string(), cert.to_string()));
        self
    }
    
    fn run(self) -> io::Result<()> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let ioc = self.ioc.clone();
            let app_config_provider = self.app_config_provider.clone();

            let app_builder = move || {
                let ioc_arc: Arc<dyn InteractorFactory> = ioc.clone();
                let ioc_data: web::Data<dyn InteractorFactory> = web::Data::from(ioc_arc);

                App::new()
                    .service(web::scope("/api")
                        .configure(presentation::web::rest::user::router)
                        .configure(presentation::web::rest::session::router)
                        .configure(presentation::web::rest::access_log::router)
                        .configure(presentation::web::rest::role::router)
                        .configure(presentation::web::rest::stats::router)
                        .configure(presentation::web::rest::permission::router)
                        .configure(presentation::web::rest::service::router)
                    )
                    .app_data(web::Data::new(
                        app_config_provider.clone()
                    ))
                    .app_data(ioc_data)
                    .default_service(web::route().to(presentation::web::exception::not_found))
                    .wrap(Logger::default())
            };

            let available_workers = {
                let connection_config = self.connection_config.lock().unwrap();
                connection_config.workers
            };

            let tcp_listener = {
                let connection_config = self.connection_config.lock().unwrap();
                match connection_config.tcp_listener.as_ref() {
                    Some(tcp_listener) => tcp_listener.try_clone().unwrap(),
                    None => {
                        log::error!("TcpListener is not set");
                        std::process::exit(1);
                    },
                }
            };
            
            let tls = {
                let connection_config = self.connection_config.lock().unwrap();
                connection_config.tls.clone()
            };

            let mut server = ActixHttpServer::new(app_builder);
            if let Some(tls) = tls {
                let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                match tls_builder.set_private_key_file(tls.0, SslFiletype::PEM) {
                    Ok(_) => {},
                    Err(error) => {
                        log::error!("Failed to set private key file: {}", error.to_string());
                        std::process::exit(1);
                    },
                }
                match tls_builder.set_certificate_chain_file(tls.1) {
                    Ok(_) => {},
                    Err(error) => {
                        log::error!("Failed to set certificate chain file: {}", error.to_string());
                        std::process::exit(1);
                    },
                };
                server = server.listen_openssl(tcp_listener, tls_builder).unwrap();
            } else {
                server = server.listen(tcp_listener).unwrap();
            }

            server = server
                .keep_alive(KeepAlive::Timeout(Duration::from_secs(100))) // TODO: Поэкспериментировать с gateway
                .workers(available_workers);

            server.addrs_with_scheme().iter().for_each(|addr| {
                let (socket_addr, str_ref) = addr;
                log::info!("🚀 Http Server started at {}://{:?}", str_ref, socket_addr);
            });

            server.run().await.and_then(|_| {
                log::info!("Http Server stopped!");
                Ok(())
            }).unwrap();
        });
        Ok(())
    }
}