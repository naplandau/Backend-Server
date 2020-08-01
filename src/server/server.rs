use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use actix_web::http::{header::{CONTENT_TYPE,AUTHORIZATION}, HeaderValue};
use listenfd::ListenFd;
use actix_cors::Cors;
use dotenv;

use crate::app::routes::routes::routes;
use crate::config::config::CONFIG;

pub async fn server() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //Create server state
    //let data = new_state::<String>();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move ||{
        App::new()
        //.configure(add_cache)
        //.wrap(Cors::new().supports_credentials().finish())
        .wrap(Logger::default())
        .wrap(Cors::new()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600)
                .finish())
        //.wrap(get_identity_service())
        //.configure(add_pool)
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