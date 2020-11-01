pub use crate::config::config::CONFIG;
pub use crate::core::db::*;
pub use crate::core::models::response::*;
pub use crate::services::email_service::*;
pub use crate::core::errors::Error;
// pub use crate::utils::handlers::hasher::{hash_validation, HASHER};
pub use actix_web::{
    http::StatusCode, web, web::Json, HttpRequest, HttpResponse, Responder, ResponseError,
};
pub use bson::{doc, Bson, Document};
pub use chrono::{DateTime, Duration, FixedOffset, Local, TimeZone, Utc};
pub use futures::StreamExt;
pub use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
pub use lettre_email::Email;
pub use mongodb::{options::*, Cursor};
// pub use regex::Regex;
// pub use rand;
pub use uuid::Uuid;
pub use validator::Validate;
pub use std::collections::HashMap;
