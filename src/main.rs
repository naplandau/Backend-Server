#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
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
mod services;
#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod middleware;
#[allow(dead_code)]
mod chatter;

use actix_web::{
    http,dev,
    middleware::errhandlers::{ErrorHandlerResponse},
    Result as ActixResult,
};
use actix::fut::err;
use actix_web::web::resource;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use crate::app::routes;
    use crate::config::config::CONFIG;

    // use actix_cors::Cors;
    use actix_session::{CookieSession, Session};
    //use actix_identity::{CookieIdentityPolicy, IdentityService};
    // use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
    use actix_web::{
        middleware::{errhandlers::ErrorHandlers, Compress, Logger},
        web, App, HttpResponse, HttpServer,
    };
    use actix_files as fs;
    use listenfd::ListenFd;
    use env_logger::Env;
    // use rand::Rng;

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    core::db::get_mongo().await;
    //let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let mut server = HttpServer::new(move || {
        App::new()
            //.configure(add_cache)
            // .app_data(web::JsonConfig::default()
            //     // register error_handler for JSON extractors.
            //     .error_handler(utils::handlers::json_error_handler))
            .wrap(Logger::default())
            .wrap(Compress::new(http::ContentEncoding::Br))
            .data(web::JsonConfig::default().limit(4096).error_handler(|err, _req| {
                println!("Json parse fail!: {:?}", err);
                actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            }))
            // .wrap(middleware::read_response_body::Logging)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
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
            //     Cors::default()
            //         .allowed_origin("*")
            //         .send_wildcard()
            //         .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
            //         .supports_credentials()
            //         .max_age(3600)
            // )

            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .configure(routes::init_route)
            .service(web::resource("/chat/").route(web::get().to(chatter::chat::chat)))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
            .service(fs::Files::new("/","static/").index_file("index.html"))
    });

    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };
    server.run().await
}

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> ActixResult<ErrorHandlerResponse<B>> {
//     res.response_mut().headers_mut().insert(
//         http::header::CONTENT_TYPE,
//         http::HeaderValue::from_static("Error"),
//     );
//     Ok(ErrorHandlerResponse::Response(res))
// }
