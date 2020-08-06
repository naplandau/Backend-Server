#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate env_logger;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

use actix_web::{middleware, web, web::Data, App, HttpServer, HttpResponse};
use actix_web::http::{header::{CONTENT_TYPE,AUTHORIZATION}, HeaderValue};
use listenfd::ListenFd;
use actix_cors::Cors;
use dotenv;

use mongodb::{Client, options::ClientOptions};
use std::sync::*;
use bson::doc;
use std::env;
use crate::app::routes::routes::routes;
use crate::config::config::CONFIG;
use crate::core::db::db::{app_state, init_pgpool, init_mongopool};

mod server;
mod app;
mod config;
mod core;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut listenfd = ListenFd::from_env();
    //mongodb connection
    let pool = init_mongopool(database_url).await.expect("Failed to create pool");
    
    // test ping mongo
    // let doc = pool.lock().unwrap().database("admin").run_command(doc!{"ping":1},None).await.unwrap();
    // println!("{}",doc);

    //postgresql connection
    //let pool = init_pgpool(database_url).expect("Failed to create pool");

    let mut server = HttpServer::new(move ||{
        App::new()
        .app_data(pool.clone())
        //.configure(add_cache)
        //.wrap(Cors::new().supports_credentials().finish())
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::default())
        .wrap(Cors::new()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600)
                .finish())
        //.configure(add_pool)
        //.wrap(get_identity_service())
        //.app_data(data.clone())
        .configure(routes)
        .default_service(web::route().to(||HttpResponse::NotFound()))
    });

    server = if let Some(l) =listenfd.take_tcp_listener(0)?{
        server.listen(l)?
    }else{
        server.bind(&CONFIG.server)?
    };
    server.run().await
}
