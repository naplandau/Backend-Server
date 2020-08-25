use actix_web::web;
use actix_web::{HttpResponse, Responder};

async fn index1() -> impl Responder {
    HttpResponse::Ok().body("API ARTICLE GET")
}
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("API ARTICLE POST")
}
async fn index3(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("API ARTICLE GET DETAIL".to_owned()+&*id.to_string())
}
pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("articles")
            .route(web::get().to(index1))
            .route(web::post().to(index2)),
    )
    .service(web::resource("articles/{id}").route(web::get().to(index3)));
}
