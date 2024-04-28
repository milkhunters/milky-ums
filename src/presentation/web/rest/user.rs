use actix_web::{delete, get, HttpResponse, post, put, Responder, Result, web};

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::user::get_by_id::GetUserByIdDTO;
use crate::ioc::IoC;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // .service(users_range)
            .service(user_by_id)
            .service(users_by_ids)
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

#[get("")]
async fn user_by_id(
    payload: web::Query<GetUserByIdDTO>,
    ioc: web::Data<IoC>,
) -> Result<HttpResponse, ApplicationError> {
    match ioc.get_user_by_id().execute(payload.into_inner()).await {
        Ok(user) => {
            Ok(HttpResponse::Ok().json(user))
        },
        Err(e) => return Err(e),
    }
}

#[get("")]
async fn users_by_ids() -> impl Responder {
    HttpResponse::Ok().body("get_users")
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