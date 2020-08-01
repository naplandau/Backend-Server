use actix_web::{web, guard, HttpResponse, Responder};
use crate::app::modules::health::get_health;
//use futures::Future;

async fn index() -> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(web::resource("/").to(index))
        .service(web::resource("/health").to(get_health))
        .service(web::scope("/api/v1").guard(guard::Header("content-type", "application/json"))
            .service(web::resource("users")
                .route(web::get().to(get_health))
            )
            .service(web::resource("login")
                .route(web::get().to(||{HttpResponse::Ok()}))
            )    
        );         

}