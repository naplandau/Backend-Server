use actix_web::web;
use actix_web::{Responder, HttpResponse};

async fn index1() -> impl Responder {
    HttpResponse::Ok().body("API USER GET")
}
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("API USER POST")
}
pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("users")
            .route(web::get().to(index1))
            .route(web::post().to(index2)),
    );
}
