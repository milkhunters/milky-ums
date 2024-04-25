use actix_web::{HttpResponse, post, Responder, web};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/role")
            .service(create_role)
    );
}

#[post("")]
async fn create_role() -> impl Responder {
    HttpResponse::Ok().body("create_role")
}