use actix_web::{get, HttpRequest, HttpResponse, patch, post, put, Result, web};
use actix_web::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppConfigProvider;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::user::change_password::ChangePasswordDTO;
use crate::application::user::confirm::ConfirmUserDTO;
use crate::application::user::create::CreateUserDTO;
use crate::application::user::get_by_id::GetUserByIdDTO;
use crate::application::user::get_by_ids::GetUsersByIdsDTO;
use crate::application::user::get_range::GetUserRangeDTO;
use crate::application::user::send_confirm_code::SendConfirmCodeDTO;
use crate::application::user::update::UpdateUserDTO;
use crate::application::user::update_self::UpdateSelfDTO;
use crate::presentation::id_provider::make_id_provider_from_request;
use crate::presentation::interactor_factory::InteractorFactory;
use crate::presentation::web::deserializers::deserialize_uuid_list;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(users_by_query)
            .service(user_self)
            .service(create_user)
            .service(update_user)
            .service(update_user_self)
            .service(change_password_self)
            .service(
                web::scope("/confirm")
                    .service(confirm_email)
            )
    );
}

#[derive(Debug, Deserialize)]
struct UsersQuery {
    id: Option<Uuid>,
    #[serde(deserialize_with = "deserialize_uuid_list", default)]
    ids: Option<Vec<Uuid>>,
    page: Option<u64>,
    per_page: Option<u64>

}

#[get("")]
async fn users_by_query(
    data: web::Query<UsersQuery>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );

    if let Some(id) = &data.id {
        let data = ioc.get_user_by_id(id_provider).execute(
            GetUserByIdDTO { id: id.clone() }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let Some(ids) = &data.ids {
        let data = ioc.get_users_by_ids(id_provider).execute(
            GetUsersByIdsDTO { ids: ids.clone(), }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let (Some(page), Some(per_page)) = (&data.page, &data.per_page) {
        let data = ioc.get_user_range(id_provider).execute(
            GetUserRangeDTO {
                page: page.clone(),
                per_page: per_page.clone()
            }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    }
    Err(ApplicationError::InvalidData(ErrorContent::Message("Invalid query".to_string())))
}

#[get("/self")]
async fn user_self(
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.get_user_self(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
async fn create_user(
    data: web::Json<CreateUserDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.create_user(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[put("")]
async fn update_user(
    data: web::Json<UpdateUserDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.update_user(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[put("self")]
async fn update_user_self(
    data: web::Json<UpdateSelfDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.update_user_self(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[patch("self/password")]
async fn change_password_self(
    data: web::Json<ChangePasswordDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    ioc.change_password(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
}

#[derive(Debug, Deserialize)]
struct ConfirmQuery {
    code: Option<u32>
}

#[post("/{email}")]
async fn confirm_email(
    email: web::Path<String>,
    query: web::Query<ConfirmQuery>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    
    let data = match query.code {
        Some(code) => {
            let data = ConfirmUserDTO {
                email: email.into_inner(),
                code
            };
            ioc.confirm_user(id_provider).execute(data).await?
        }
        None => {
            let data = SendConfirmCodeDTO {
                email: email.into_inner(),
            };
            ioc.send_confirm_code(id_provider).execute(data).await?
        }
    };
    
    Ok(HttpResponse::Ok().json(data))
}