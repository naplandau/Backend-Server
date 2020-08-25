pub mod routes;
pub mod users_route;
pub mod article_route;

use actix_web::guard;
use actix_web::web;

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .guard(guard::Header("content-type", "application/json"))
            .configure(users_route::init_route)
            .configure(article_route::init_route)
    );
}
