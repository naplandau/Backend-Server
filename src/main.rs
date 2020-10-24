#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
// #[macro_use]
// extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate validator_derive;

#[allow(dead_code)]
mod app;
#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod core;
#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use crate::app::routes;
    use crate::config::config::CONFIG;

    // use actix_cors::Cors;
    //use actix_session::{CookieSession, Session};
    //use actix_identity::{CookieIdentityPolicy, IdentityService};
    // use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use actix_web::{middleware, web, App, HttpResponse, HttpServer};
    use listenfd::ListenFd;
    // use rand::Rng;

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG","actix_web=debug,actix_server=info");
    //std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    core::db::get_mongo().await;
    //let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let mut server = HttpServer::new(move || {
        App::new()
            //.configure(add_cache)
            // .app_data(web::JsonConfig::default()
            //     // register error_handler for JSON extractors.
            //     .error_handler(utils::handlers::json_error_handler),)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .data(web::JsonConfig::default().limit(4096))
            //.wrap(CookieSession::signed(&[0; 32]).secure(false))
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&private_key)
            //     .name("auth")
            //     .path("/")
            //     .domain(&CONFIG.domain)
            //     //.max_age_time(chrono::Duration::days(1))
            //     .max_age(86400)
            //     .secure(false),
            // ))
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("*")
            //         .send_wildcard()
            //         .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
            //         .supports_credentials()
            //         .max_age(3600)
            //         .finish(),
            // )
            .configure(routes::init_route)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    });

    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };
    server.run().await
}
