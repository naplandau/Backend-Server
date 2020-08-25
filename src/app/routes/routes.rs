use crate::app::modules::health::get_health;
use crate::app::modules::*;
use actix_web::{guard, web, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(index))
        .service(web::resource("/check_auth").to(cb_users::check_auth))
        .service(web::resource("/admin").to(cb_users::admin))
        .service(web::resource("/mongo").to(cb_users::test_mongo))
        .service(web::resource("/login").to(cb_users::login))
        .service(web::resource("/register").to(cb_users::register))
        .service(web::resource("/forgot").to(cb_users::test_mongo))
        .service(web::resource("/health").to(get_health))
        .service(web::resource("/confirmation?").to(get_health));
    // .service(
    //     web::scope("/api/v1")
    //         .guard(guard::Header("content-type", "application/json"))
    //         .service(
    //             web::resource("users")
    //                 .route(web::get().to(cb_users::get_users))
    //                 .route(web::post().to(cb_users::create_user)),
    //         )
    //         .service(
    //             web::resource("users/{id}")
    //                 .route(web::get().to(cb_users::get_user))
    //                 .route(web::put().to(cb_users::update_user)),
    //         )
    //         .service(
    //             web::resource("users/confirmation/{id}")
    //                 .route(web::put().to(cb_users::)),
    //         )
    //         .service(
    //             web::resource("users/forgot")
    //                 .route(web::post().to(cb_users::)),
    //         )
    //         .service(web::resource("login").route(web::get().to(|| HttpResponse::Ok()))),
    // );
}
