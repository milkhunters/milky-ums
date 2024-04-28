use actix_web::{delete, get, HttpResponse, post, Responder, web};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sessions")
            .service(sessions_range_self)
            .service(sessions_range_by_user_id)
            .service(create_session)
            .service(refresh_current_session)
            .service(logout_current_session)
            .service(logout_session_by_id)
    );
}

#[get("")]
async fn sessions_range_self() -> impl Responder {
    HttpResponse::Ok().body("sessions")
}

#[get("")]
async fn sessions_range_by_user_id(
    id: web::Query<String>
) -> impl Responder {
    HttpResponse::Ok().body(format!("sessions by {}", id))
}


#[post("")]
async fn create_session(
    id: web::Query<String>
) -> impl Responder {
    HttpResponse::Ok().body(format!("sessions by {}", id))
}

#[post("")]
async fn refresh_current_session(
    id: web::Query<String>
) -> impl Responder {
    HttpResponse::Ok().body(format!("sessions by {}", id))
}


#[delete("")]
async fn logout_current_session() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

#[delete("")]
async fn logout_session_by_id(
    id: web::Query<String>
) -> impl Responder {
    HttpResponse::Ok().body("logout")
}
