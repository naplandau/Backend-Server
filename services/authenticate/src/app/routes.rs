use actix_web::{web};
pub fn init_route(cfg: &mut web::ServiceConfig) {
    use crate::app::controllers::*;
    cfg.service(
        web::scope("/api/v1")
            // .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("auth").route(web::post().to(nats_client::create_users))),
    );
}
