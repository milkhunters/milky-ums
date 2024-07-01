use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::error;
use tonic::transport::Server as TonicGrpcServer;

use crate::application::common::server::{ConnectionConfig, Server};
use crate::ioc::IoC;
use crate::presentation::grpc::greeter::proto::ums_control_server::UmsControlServer;
use crate::presentation::grpc::greeter::UMSGreeter;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct GrpcServer {
    connection_config: Arc<Mutex<ConnectionConfig>>,
    service_name: String,
    ioc: Arc<dyn InteractorFactory>
}

impl GrpcServer {
    pub fn new(
        service_name: String,
        ioc: IoC
    ) -> Self {

        let ioc_arc: Arc<dyn InteractorFactory> = Arc::new(ioc);

        Self {
            connection_config: Arc::new(Mutex::new(ConnectionConfig::default())),
            service_name,
            ioc: ioc_arc,
        }
    }
}

impl Server for GrpcServer {
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
        let available_workers = {
            let connection_config = self.connection_config.lock().unwrap();
            connection_config.workers
        };
        
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(available_workers)
            .enable_all()
            .build()
            .unwrap();
        
        rt.block_on(async {
            let ioc = self.ioc.clone();

            let socket_addr = {
                let mut connection_config = self.connection_config.lock().unwrap();
                let addr = match connection_config.tcp_listener.as_ref() {
                    Some(tcp_listener) => tcp_listener
                        .try_clone().unwrap()
                        .local_addr().unwrap(),
                    None => {
                        log::error!("TcpListener is not set");
                        std::process::exit(1);
                    },
                };
                connection_config.tcp_listener = None;
                addr
            };

            let tls = {
                let connection_config = self.connection_config.lock().unwrap();
                connection_config.tls.clone()
            };
            
            if tls.is_some() {
                error!("TLS is not supported yet");
                std::process::exit(1);
            }
            
            let ums_greeter = UMSGreeter::new(ioc, self.service_name.clone());

            log::info!("🚀 gRPC Server started at {}", socket_addr);
            TonicGrpcServer::builder()
                .http2_keepalive_interval(Some(Duration::from_secs(100)))
                .http2_keepalive_timeout(Some(Duration::from_secs(10)))
                .concurrency_limit_per_connection(10000)
                .add_service(UmsControlServer::new(ums_greeter))
                .serve(socket_addr.clone())
                .await.and_then(|_| {
                    log::info!("Grpc Server stopped!");
                    Ok(())
                }).unwrap();
        });
        Ok(())
    }
}