use actix_web::{delete, get, HttpResponse, post, put, Responder, Result, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::user::get_by_id::GetUserByIdDTO;
use crate::application::user::get_by_ids::GetUsersByIdsDTO;
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


// #[get("")]
// async fn users_range() -> impl Responder {
//     HttpResponse::Ok().body("get_users")
// }


#[derive(Debug, Deserialize)]
struct UserByIdsData {
    #[serde(deserialize_with = "deserialize_uuid_list")]
    ids: Vec<Uuid>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum UsersQuery {
    Id(GetUserByIdDTO),
    Ids(UserByIdsData)
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
        }
    };
}

#[get("/self")]
async fn user_self() -> impl Responder {
    HttpResponse::Ok().body("self")
}


#[post("")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("create_user")
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