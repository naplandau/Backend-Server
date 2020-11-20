use actix_web::{web, HttpResponse, guard};
use super::lib::*;
pub fn init_route(cfg: &mut web::ServiceConfig) {
    use crate::app::modules::users::*;
    cfg.service(
        web::scope("/api/v1")
            .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("health").to(get_health))
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
            .service(web::resource("admin").to(admin))
            .service(web::resource("auth").to(login)),
    );
}
#[derive(Serialize)]
struct HealthResponse {
    pub status: String,
    pub version: String,
}

async fn get_health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok".into(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}