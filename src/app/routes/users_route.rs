use crate::app::modules::users::*;
use actix_web::web;
pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("users")
            .route(web::get().to(get_users))
            .route(web::post().to(create_users))
            .route(web::delete().to(delete_users))
            // .route(web::put().to(|| ))
    )
    .service(
        web::resource("users/{id}")
            .route(web::get().to(get_user))
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user))
            .route(web::delete().to(find_delete_user))
    )
    // .service(web::resource("register/{id}").to(verify_register))
    // .service(web::resource("login").route(web::post().to(login)))
    .service(web::resource("admin").to(admin));
}
