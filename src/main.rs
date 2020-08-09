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
extern crate validator;
#[macro_use] 
extern crate bson;

use actix_cors::Cors;
use actix_web::http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    //HeaderValue,
};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv;
use listenfd::ListenFd;

use crate::app::routes::routes::routes;
use crate::config::config::CONFIG;
use std::env;
use crate::core::db::get_mongo;

mod app;
mod config;
mod core;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    //std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    get_mongo().await.unwrap();
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            //.configure(add_cache)
            //.wrap(Cors::new().supports_credentials().finish())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("*")
            //         .send_wildcard()
            //         .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
            //         .max_age(3600)
            //         .finish(),
            // )
            //.wrap(get_identity_service())
            .configure(routes)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };
    server.run().await
}
