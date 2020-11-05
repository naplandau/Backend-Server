use crate::app::modules::users::*;
use crate::middleware;
use actix_web::{web, HttpResponse};

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("users")
            .route(web::get().to(get_users))
            // .wrap(middleware::read_request_body::Logging)
            .route(web::post().to(create_users))
            .route(web::delete().to(delete_users))
            .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())), // .route(web::put().to(|| ))
    )
    .service(
        web::resource("users/{id}")
            .route(web::get().to(get_user))
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user))
            // .route(web::delete().to(find_delete_user))
            .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())),
    )
    .service(web::resource("admin").to(admin))
    .service(web::resource("auth").to(login));
}
