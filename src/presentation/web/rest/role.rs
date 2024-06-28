use actix_web::{get, HttpRequest, HttpResponse, Result, post, web, put};
use serde::Deserialize;

use crate::AppConfigProvider;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::role::create::CreateRoleDTO;
use crate::application::role::get_by_id::GetRoleByIdDTO;
use crate::application::role::get_by_ids::GetRolesByIdsDTO;
use crate::application::role::get_range::RoleRangeDTO;
use crate::application::role::update::UpdateRoleDTO;
use crate::domain::models::role::RoleId;
use crate::presentation::id_provider::make_id_provider_from_request;
use crate::presentation::interactor_factory::InteractorFactory;
use crate::presentation::web::deserializers::deserialize_uuid_list;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .service(create_role)
            .service(get_roles)
            .service(update_role)
    );
}

#[post("")]
async fn create_role(
    data: web::Json<CreateRoleDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.create_role(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}


#[derive(Debug, Deserialize)]
struct RolesQuery {
    id: Option<RoleId>,
    #[serde(deserialize_with = "deserialize_uuid_list", default)]
    ids: Option<Vec<RoleId>>,
    page: Option<u64>,
    per_page: Option<u64>
}

#[get("")]
async fn get_roles(
    data: web::Query<RolesQuery>,
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
        let data = ioc.get_role_by_id(id_provider).execute(
            GetRoleByIdDTO { id: id.clone() }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let Some(ids) = &data.ids {
        let data = ioc.get_roles_by_ids(id_provider).execute(
            GetRolesByIdsDTO { ids: ids.clone(), }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let (Some(page), Some(per_page)) = (&data.page, &data.per_page) {
        let data = ioc.get_role_range(id_provider).execute(
            RoleRangeDTO {
                page: page.clone(),
                per_page: per_page.clone()
            }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    }
    Err(ApplicationError::InvalidData(ErrorContent::Message("Invalid query".to_string())))
}

#[put("")]
async fn update_role(
    data: web::Json<UpdateRoleDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    app_config_provider: web::Data<AppConfigProvider>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_id_provider_from_request(
        &app_config_provider.service_name,
        app_config_provider.is_intermediate,
        &req
    );
    let data = ioc.update_role(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}