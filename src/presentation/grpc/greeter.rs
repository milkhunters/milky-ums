use std::sync::Arc;

use tonic::{Request, Response, Status};
use tonic::codegen::http::HeaderMap;

use proto::{EpRequest, EpResponse};
use proto::ums_control_server::UmsControl;
use crate::application::common::interactor::Interactor;

use crate::application::service::sync::ServiceSyncDTO;
use crate::domain::models::service::ServiceTextId;
use crate::presentation::grpc::greeter::proto::{PermissionsList, SsRequest};
use crate::presentation::id_provider::make_id_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub mod proto {
    tonic::include_proto!("ums.control");
}

pub struct UMSGreeter {
    ioc: Arc<dyn InteractorFactory>,
    service_text_id: ServiceTextId,
}

impl UMSGreeter {
    pub fn new(ioc: Arc<dyn InteractorFactory>, service_text_id: ServiceTextId) -> Self {
        Self {
            ioc,
            service_text_id,
        }
    }
}


#[tonic::async_trait]
impl UmsControl for UMSGreeter {
    async fn extract_payload(&self, request: Request<EpRequest>) -> Result<Response<EpResponse>, Status> {
        let payload = request.get_ref();
        
        let user_agent = payload.user_agent.clone();
        let user_ip = payload.user_ip.clone();
        let session_token = payload.session_token.clone();
        
        let id_provider = make_id_provider(
            &self.service_text_id,
            None,
            Some(user_agent),
            &user_ip
        );

        let resp = self.ioc.extract_payload(id_provider)
            .execute(session_token).await;
        
        match resp {
            Ok(data) => {
                Ok(Response::new(EpResponse {
                    session_id: String::from(data.session_id),
                    user_id: String::from(data.user_id),
                    user_state: data.user_state.to_string(),
                    permissions: data.permissions.iter().map(|(k, v)| {
                        (String::from(k), PermissionsList { permission_text_ids: v.clone() })
                    }).collect()
                }))
            },
            Err(error) => {
                let mut header = HeaderMap::new();
                header.insert("Content-Type", "application/json".parse().unwrap());

                let status = Status::unauthenticated(
                    serde_json::to_string(&error.as_json()).unwrap()
                );
                status.add_header(&mut header).unwrap();

                Err(status)
            }
        }
    }

    async fn sync_service(&self, request: Request<SsRequest>) -> Result<Response<()>, Status> {
        let payload = request.get_ref();
        
        let service_text_id = payload.text_id.clone();
        let permission_text_ids = payload.permission_text_ids.clone();

        let executor = self.ioc.sync_service();
        
        let resp = executor.execute(
            ServiceSyncDTO {
                service_text_id,
                permission_text_ids
            }
        ).await;

        match resp {
            Ok(_) => Ok(Response::new(())),
            Err(error) => Err(Status::internal(error.to_string()))
        }
    }
}
