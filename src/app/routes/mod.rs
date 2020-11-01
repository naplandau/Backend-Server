mod article_route;
mod users_route;

use crate::app::modules::health;
// use actix_web::guard;
use actix_web::web;

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("health").to(health::get_health))
            .configure(users_route::init_route)
            .configure(article_route::init_route)
    );
}
