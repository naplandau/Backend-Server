#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate env_logger;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate bson;

use crate::app::routes;
use crate::config::config::CONFIG;
use crate::core::db::get_mongo;

mod app;
mod config;
mod core;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use actix_web::{middleware, web, App, HttpResponse, HttpServer};
    use dotenv;
    use listenfd::ListenFd;

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    //std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    get_mongo().await.unwrap();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            //.configure(add_cache)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .data(web::JsonConfig::default().limit(4096))
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(secret_key.as_bytes())
            //     .name("auth")
            //     .path("/")
            //     .domain(domain.as_str())
            //     //.max_age_time(chrono::Duration::days(1))
            //     .secure(false),
            // ))
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("*")
            //         .send_wildcard()
            //         .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
            //         .max_age(3600)
            //         .finish(),
            // )
            //.wrap(get_identity_service())
            .configure(routes::init_route)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };
    server.run().await
}
