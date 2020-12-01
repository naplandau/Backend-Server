pub use crate::config::CONFIG;
pub use crate::core::*;
pub use crate::errors::ServerError;
pub use crate::models::*;
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
pub use std::time::Instant;
pub use validator::Validate;
pub fn get_sub_field(doc: &serde_json::Value) -> serde_json::Value {
    let mut new_doc = doc.clone();
    let keys = vec![
        "password",
        "created_by",
        "created_time_dt",
        "updated_by",
        "updated_time_dt",
    ];
    for key in keys.iter() {
        new_doc[key].take();
    }
    new_doc
}
