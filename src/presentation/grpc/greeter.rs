use std::sync::{Arc, Mutex};
use log::info;
use tonic::{Request, Response, Status};

use proto::{EpResponse, EpRequest};

use proto::ums_control_server::UmsControl;
use crate::application::common::interactor::Interactor;
use crate::domain::models::service::ServiceTextId;
use crate::presentation::grpc::greeter::proto::PermissionsList;
use crate::presentation::id_provider::make_id_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub mod proto {
    tonic::include_proto!("ums.control");
}

pub struct UMSGreeter {
    ioc: Arc<dyn InteractorFactory>,
    service_name: ServiceTextId,
}

impl UMSGreeter {
    pub fn new(ioc: Arc<dyn InteractorFactory>, service_name: ServiceTextId) -> Self {
        Self {
            ioc,
            service_name,
        }
    }
}


#[tonic::async_trait]
impl UmsControl for UMSGreeter {
    async fn extract_payload(&self, request: Request<EpRequest>) -> Result<Response<EpResponse>, Status> {
        // let start = std::time::Instant::now();
        let payload = request.get_ref();
        
        let user_agent = payload.user_agent.clone();
        let user_ip = payload.user_ip.clone();
        let session_token = payload.session_token.clone();
        
        let id_provider = make_id_provider(
            &self.service_name,
            None,
            Some(user_agent),
            &user_ip
        );

        let resp = self.ioc.extract_payload(id_provider)
            .execute(session_token).await;
        // info!("Elapsed exec time: {:?}", start.elapsed());
        match resp {
            Ok(data) => {
                Ok(Response::new(EpResponse {
                    session_id: String::from(data.session_id),
                    user_id: String::from(data.user_id),
                    user_state: data.user_state.to_string(),
                    permissions: data.permissions.iter().map(|(k, v)| {
                        (String::from(k), PermissionsList { permission: v.clone() })
                    }).collect()
                }))
            },
            Err(error) => Err(Status::unauthenticated(error.to_string()))
        }
    }
}
