#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
// #[macro_use]
// extern crate log;

#[allow(dead_code)]
mod app;
#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod core;
#[allow(dead_code)]
mod errors;
#[allow(dead_code)]
mod middleware;
#[allow(dead_code)]
mod models;
#[allow(dead_code)]
mod nats_broker;
#[allow(dead_code)]
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use crate::core::nats_server;

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    nats_server::nats_server().await; //Start Nats server
    let mut server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(app::routes::init_route)
            .default_service(actix_web::web::route().to(|| actix_web::HttpResponse::MethodNotAllowed()))
    });
    let mut listenfd = listenfd::ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&config::CONFIG.server)?
    };
    server.run().await
}
