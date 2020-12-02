use actix_web::{web, HttpResponse};
use bson::doc;
pub fn init_route(cfg: &mut web::ServiceConfig) {
    use super::lib::*;
    use crate::app::controllers::*;
    cfg.service(
        web::scope("/api/v1")
            // .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("get").to(get_health))
            .service(web::resource("set").to(set_health))
            .service(
                web::resource("users")
                    .route(web::get().to(get_users))
                    // .wrap(middleware::read_request_body::Logging)
                    .route(web::post().to(create_users))
                    .route(web::delete().to(delete_users))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())), // .route(web::put().to(|| ))
            )
            .service(
                web::resource("users/{id}")
                    .route(web::get().to(get_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
                    // .route(web::delete().to(find_delete_user))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())),
            )
            .service(web::resource("nats/users").route(web::post().to(nats_client::create_users)))
            // .service(web::resource("admin").to(admin))
            // .service(web::resource("auth").to(login)),
    );
}
#[derive(Serialize)]
struct HealthResponse {
    pub status: String,
    pub version: String,
}
use crate::{core::redis_db::*};
use crate::nats_broker::*;
use crate::rabbit_queue::*;
// use lapin::{
//     options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties,
// };
async fn get_health(
    _pool: web::Data<RedisFactory>,
    _queue_pool: web::Data<RabbitPool>,
    _nats_pool: web::Data<NatsConnection>,
) -> HttpResponse {
    // let conn = pool.get_connection().await.expect("");
    // let res = get_str(&pool.pool, "abc").await.unwrap();
    // let conn = _queue_pool.get().await.expect("msg");
    // let channel = conn.create_channel().await.unwrap();
    // let _ = channel
    //     .queue_declare(
    //         "ha_qu_test",
    //         QueueDeclareOptions::default(),
    //         FieldTable::default(),
    //     )
    //     .await
    //     .unwrap();
    // let _ = channel
    //     .queue_declare(
    //         "ha_qu_test1",
    //         QueueDeclareOptions::default(),
    //         FieldTable::default(),
    //     )
    //     .await
    //     .unwrap();
    // let send_props = BasicProperties::default().with_kind(format!("Sender: ").into());
    // let res = channel
    //     .basic_publish(
    //         "",
    //         "ha_qu_test",
    //         BasicPublishOptions::default(),
    //         b"haha".to_vec(),
    //         send_props.clone(),
    //     )
    //     .await
    //     .unwrap()
    //     .await
    //     .unwrap();

    // let res1 = channel
    //     .basic_publish(
    //         "",
    //         "ha_qu_test1",
    //         BasicPublishOptions::default(),
    //         b"huhu".to_vec(),
    //         send_props,
    //     )
    //     .await
    //     .unwrap()
    //     .await
    //     .unwrap();
    // let a = match res1 {
    //     Confirmation::NotRequested => "NotRequested",
    //     _ => "ABC",
    // };
    // let b = match res {
    //     Confirmation::NotRequested => "NotRequested",
    //     _ => "ABC",
    // };
    HttpResponse::Ok().json(HealthResponse {
        status: "".to_owned(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
async fn set_health(pool: web::Data<RedisFactory>) -> HttpResponse {
    // let conn = pool.get_connection().await.expect("");
    let _res = set_str(&pool.pool, "abc", "1234", 0).await.unwrap();
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok".into(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
