use crate::core::db::users_db;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use crate::server::middleware::auth::AuthorizationService;
use crate::core::models::users::{Login, Register};

pub async fn login(user: web::Json<Login>) -> HttpResponse{
    let proc = users_db::login(user.into_inner()).await;
    match proc {
        Ok(_) => HttpResponse::Ok().json(proc.unwrap()),
        Err(_) => HttpResponse::Ok().status(StatusCode::from_u16(401).unwrap()).json(proc.unwrap_err()),
    }

}

pub async fn register(user: web::Json<Register>) -> HttpResponse{
    HttpResponse::Ok().json(users_db::register(user.into_inner()).await)
}