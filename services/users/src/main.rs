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
use crate::core::db::rabbit_queue::*;
use futures::StreamExt;
use lapin::{options::*};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use crate::core::db::redis_db::*;
    use actix_web::{
        http,
        middleware::{
            errhandlers::{ErrorHandlerResponse, ErrorHandlers},
            Logger,
        },
        web, App, HttpResponse, HttpServer,
    };

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let redis_fac = RedisFactory::connect(config::CONFIG.redis_url.to_owned())
        .await
        .expect("");
    let rabbit_fac = RabbitFactory::get_pool(config::CONFIG.rabbit_url.to_owned())
        .await
        .expect("");
    let rabbit_fac1 = RabbitFactory::get_pool(config::CONFIG.rabbit_url.to_owned())
        .await
        .expect("");

    rabbit_server(rabbit_fac1).await;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(redis_fac.clone())
            .data(rabbit_fac.clone())
            .wrap(Logger::default())
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
    server.run().await
}

async fn queue_consume(conn: RabbitConnection) {
    let mut consumer = RabbitServer::create_consumer(&conn, "ha_qu_test", "test")
        .await
        .unwrap();
    while let Some(delivery) = consumer.next().await {
        let (channel, delivery) = delivery.expect("error in consumer");
        println!("[{}] consume messsage: {:?}", "ha_qu_test", delivery.data);
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .expect("ack");
    }
}
async fn queue_consume1(conn: RabbitConnection) {
    let mut consumer = RabbitServer::create_consumer(&conn, "ha_qu_test1", "test")
        .await
        .unwrap();
    while let Some(delivery) = consumer.next().await {
        let (channel, delivery) = delivery.expect("error in consumer");
        println!("[{}] consume messsage: {:?}", "ha_qu_test_1", delivery.data);
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .expect("ack");
    }
}

async fn rabbit_server(rabbit_fac1: RabbitPool){
    let rabbit = rabbit_fac1.to_owned();
    let conn = rabbit.get().await.unwrap();
    actix_rt::spawn(async move{
        queue_consume(conn).await
    });
    let conn1= rabbit.get().await.unwrap();
    actix_rt::spawn(async move{
        queue_consume1(conn1).await;
    });
}
// use actix::{Actor, Context};
// struct RabbitActor {
//     consumer: Consumer,
// }
// impl Actor for RabbitActor {
//     type Context = Context<Self>;
//     fn started(&mut self, ctx: &mut Self::Context) {
//         ctx.spawn(async move {
//             while let Some(delivery) = self.consumer.next().await {
//                 let (channel, delivery) = delivery.expect("error in consumer");
//                 println!("incoming message from: {:?}", delivery.properties.kind());
//                 channel
//                     .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
//                     .await
//                     .expect("ack");
//             }
//         })
//     }
// }
