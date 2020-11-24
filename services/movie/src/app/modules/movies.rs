pub use crate::config::CONFIG;
pub use crate::core::db::rabbit_queue::*;
pub use crate::core::db::*;
pub use crate::core::errors::ServerError;
pub use crate::core::models::movie::Movie;
pub use crate::core::models::response::*;
pub use crate::utils::hasher::*;
pub use actix_web::{
    http::StatusCode, web, web::Json, HttpRequest, HttpResponse, Responder, ResponseError,
};
pub use bson::{doc, Bson, Document};
pub use chrono::{DateTime, Duration, FixedOffset, Local, TimeZone, Utc};
pub use futures::StreamExt;
pub use mobc_redis::redis::{AsyncCommands, FromRedisValue};
pub use mongodb::{options::*, Cursor};
pub use std::collections::HashMap;
pub use uuid::Uuid;
pub use validator::Validate;

pub async fn create_movie(req: web::Json<AddMovie>) -> HttpResponse {
    let movie: Movie = req.to_owned().into();
    match movie_db::insert(movie.to_owned()).await {
        Ok(_id) => HttpResponse::Created().json(Response::from(movie.to_owned())),
        Err(e) => ServerError::from(e).error_response(),
    }
}
