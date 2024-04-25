use actix_web::{get, HttpResponse, Responder, web};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stats")
            .service(health_check)
    );
}


#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

