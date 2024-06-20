use log::info;
use tonic::{Request, Response, Status};

use proto::{EpResponse, EpRequest};

use proto::ums_control_server::UmsControl;

pub mod proto {
    tonic::include_proto!("ums.control");
}

#[derive(Default)]
pub struct UMSGreeter {}

#[tonic::async_trait]
impl UmsControl for UMSGreeter {
    async fn extract_payload(&self, request: Request<EpRequest>) -> Result<Response<EpResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        
        let input_payload = request.get_ref();
        
        info!("Got a request with session_token: {:?}", input_payload.session_token);

        let session_payload = EpResponse {
            session_id: "session_id".into(),
            user_id: "user_id".into(),
            user_state: "user_state".into(),
            permissions: Default::default(),
        };
        Ok(Response::new(session_payload))
    }
}
