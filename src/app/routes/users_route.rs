use crate::app::modules::users::*;
use actix_web::web;
pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("users")
            //.route(web::get().to(get_users))
            .route(web::post().to(register)),
    )
    //.service(web::resource("register/{id}").to(verify_register))
    // .service(
    //     web::resource("users/{id}")
    //         .route(web::get().to(get_user))
    //         .route(web::put().to(get_user)),
    // )
    // .service(web::resource("login").route(web::post().to(login)))
    // .service(web::resource("admin").to(admin));
    ;
}
