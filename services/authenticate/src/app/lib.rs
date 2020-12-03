pub use crate::config::CONFIG;
pub use crate::core::*;
pub use crate::errors::ServerError;
pub use crate::models::*;
pub use crate::utils::hasher::*;
pub use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder, ResponseError};
pub use bson::{doc, Bson, Document};
pub use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
pub use futures::StreamExt;
pub use mobc_redis::redis::{AsyncCommands, FromRedisValue};
pub use mongodb::{options::*, Cursor};
pub use serde_json::Value as Json;
pub use std::collections::HashMap;
pub use std::time::Instant;
pub use uuid::Uuid;
pub use validator::Validate;
pub fn get_sub_field(doc: &serde_json::Value) -> serde_json::Value {
    let mut new_doc = doc.as_object().unwrap().clone();
    let keys_rm = vec!["password", "created_by", "updated_by"];
    let keys_parse = vec!["created_time_dt", "updated_time_dt"];
    for key in keys_rm.iter() {
        new_doc.remove(*key);
    }
    for key in keys_parse.iter() {
        new_doc.insert(
            key.to_string(),
            Json::String(
                Local
                    .timestamp(new_doc.get(*key).unwrap().as_i64().unwrap(), 0)
                    .to_string(),
            ),
        );
    }
    new_doc.into()
}
