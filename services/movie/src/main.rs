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
mod middleware;
#[allow(dead_code)]
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        http,
        middleware::{
            errhandlers::{ErrorHandlerResponse, ErrorHandlers},
            Compress, Logger,
        },
        web, App, HttpResponse, HttpServer,
    };
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        println!("Json parse fail!: {:?}", err);
                        actix_web::error::InternalError::from_response(
                            err,
                            HttpResponse::BadRequest().finish(),
                        )
                        .into()
                    }),
            )
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                |mut res| {
                    res.response_mut().headers_mut().insert(
                        http::header::CONTENT_TYPE,
                        http::HeaderValue::from_static("Error"),
                    );
                    dbg!("ErrorHandlers detect!");
                    Ok(ErrorHandlerResponse::Response(res))
                },
            ))
            .configure(app::routes::init_route)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    });
    let mut listenfd = listenfd::ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&config::CONFIG.server)?
    };
    server.run();
}
