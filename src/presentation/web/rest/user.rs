use actix_web::{delete, get, HttpResponse, post, put, Responder, Result, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::user::create_user::CreateUserDTO;
use crate::application::user::get_by_id::GetUserByIdDTO;
use crate::application::user::get_by_ids::GetUsersByIdsDTO;
use crate::application::user::get_range::GetUserRangeDTO;
use crate::ioc::IoC;
use crate::presentation::interactor_factory::InteractorFactory;
use crate::presentation::web::deserializers::deserialize_uuid_list;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // .service(users_range)
            .service(users_by_query)
            .service(user_self)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(
                web::scope("/confirmation")
                    .service(confirm_email)
            )
    );
}



/**
    Попытка реализовать что-то типа:

    ```rust
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum UsersQuery {
        Id(GetUserByIdDTO),
        Ids(UserByIdsData),
        Range(GetUserRangeDTO)
    }

    #[get("")]
    async fn users_by_query(
        data: web::Query<UsersQuery>,
        ioc: web::Data<IoC>,
    ) -> Result<HttpResponse, ApplicationError> {
        return match data.0 {
            UsersQuery::Id(content) => {
                let data = ioc.get_user_by_id().execute(content).await?;
                Ok(HttpResponse::Ok().json(data))
            },
            UsersQuery::Ids(content) => {
                let data = ioc.get_users_by_ids().execute(
                    GetUsersByIdsDTO {
                        ids: content.ids.clone(),
                    }
                ).await?;
                Ok(HttpResponse::Ok().json(data))
            },
            UsersQuery::Range(content) => {
                let data = ioc.get_user_range().execute(content).await?;
                Ok(HttpResponse::Ok().json(data))
            }
        };
    }
    ```

    Потерпела неудачу из-за нескольких параметров в варианте
 */
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
    ioc: web::Data<IoC>,
) -> Result<HttpResponse, ApplicationError> {

    if let Some(id) = &data.id {
        let data = ioc.get_user_by_id().execute(
            GetUserByIdDTO { id: id.clone() }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let Some(ids) = &data.ids {
        let data = ioc.get_users_by_ids().execute(
            GetUsersByIdsDTO { ids: ids.clone(), }
        ).await?;
        return Ok(HttpResponse::Ok().json(data))
    } else if let (Some(page), Some(per_page)) = (&data.page, &data.per_page) {
        let data = ioc.get_user_range().execute(
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
async fn user_self() -> impl Responder {
    HttpResponse::Ok().body("self")
}


#[post("")]
async fn create_user(
    data: web::Json<CreateUserDTO>,
    ioc: web::Data<IoC>,
) -> Result<HttpResponse, ApplicationError> {
    let data = ioc.create_user().execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))

}

#[put("/{id}")]
async fn update_user(
    id: web::Path<String>
) -> impl Responder {
    HttpResponse::Ok().body(format!("update_user: {}", id))
}


#[delete("/{id}")]
async fn delete_user(
    id: web::Path<String>
) -> impl Responder {
    HttpResponse::Ok().body(format!("delete_user: {}", id))
}



#[post("/{email}")]
async fn confirm_email(
    email: web::Path<String>,
    code: web::Query<Option<i32>>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("confirm_email: {} with code: {:?}", email, code))
}