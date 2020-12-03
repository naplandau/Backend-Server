use actix_web::{web};
use crate::core::db::{get_nats, get_redis};

pub fn init_route(cfg: &mut web::ServiceConfig) {
    use crate::app::controllers::*;
    cfg.service(
        web::scope("/api/v1")
        .service(web::resource("/health").to(get_health))
            // .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("auth").route(web::post().to(nats_client::create_users))),
    );
}
async fn get_health() -> actix_web::HttpResponse{
    let nats = get_nats().await.unwrap();
    nats.publish("auth.token.check", "");
    nats.publish("auth.token.check_gen", "");
    nats.publish("auth.login", "");
    nats.publish("auth.register", "");
    nats.publish("auth.forgot.123", "");
    nats.publish("auth.forgot.OOO", "");
    nats.publish("auth.permission.modify", "");
    nats.publish("auth.permission.check", "");
    nats.publish("auth.token.check", "");
    nats.publish("auth.register", "");
    actix_web::HttpResponse::Ok().finish()
}
    