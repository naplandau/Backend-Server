use crate::app::modules::users::*;
use actix_web::web;
pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("users")
            .route(web::get().to(get_all_users))
            .route(web::post().to(register)),
    )
    .service(
        web::resource("users/{id}")
            .route(web::get().to(get_all_users))
            .route(web::put().to(get_all_users)),
    )
    .service(web::resource("admin").to(admin));
}
