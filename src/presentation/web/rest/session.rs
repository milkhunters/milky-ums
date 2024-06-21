use actix_web::{delete, get, HttpRequest, HttpResponse, post, Result, web};
use actix_web::cookie::Cookie;

use crate::AppConfigProvider;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::session::create::CreateSessionDTO;
use crate::application::session::delete::DeleteSessionDTO;
use crate::application::session::get_by_id::GetSessionByIdDTO;
use crate::application::session::get_by_user_id::GetSessionsByUserIdDTO;
use crate::presentation::id_provider::make_id_provider_from_request;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sessions")
            .service(sessions_self)
            .service(sessions_by_user_id)
            .service(create_session)
            .service(sessions_by_id)
            .service(delete_session)
            .service(delete_self_session)
    );
}

#[post("")]
async fn create_session(
    data: web::Json<CreateSessionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let (data, session_token) = ioc.create_session(id_provider).execute(
        data.into_inner()
    ).await?;
    
    let mut response = HttpResponse::Ok().json(data);
    response.add_cookie(
        &Cookie::build("session_token", session_token.to_string())
            .path("/")
            .http_only(true)
            .finish()
    ).unwrap();
    
    Ok(response)
}

#[delete("{id}")]
async fn delete_session(
    id: web::Path<DeleteSessionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.delete_session(id_provider).execute(
        id.into_inner()
    ).await?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("self")]
async fn delete_self_session(
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.delete_self_session(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("{id}")]
async fn sessions_by_id(
    id: web::Path<GetSessionByIdDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.get_session_by_id(id_provider).execute(
        id.into_inner()
    ).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("")]
async fn sessions_by_user_id(
    id: web::Query<GetSessionsByUserIdDTO>,
    app_config_provider: web::Data<AppConfigProvider>,
    ioc: web::Data<dyn InteractorFactory>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.get_sessions_by_user_id(id_provider).execute(
        id.into_inner()
    ).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("self")]
async fn sessions_self(
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError>{
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.get_sessions_self(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}
