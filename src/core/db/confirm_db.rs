use super::db_utils;
use crate::config::config::CONFIG;
use crate::core::models::{api_response::*, users::*};
use crate::server::handlers::hasher::{hash_validation, HASHER};
use bson::doc;
use bson::{Bson, Document};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{error::Error, options::FindOptions};
use uuid::Uuid;
const COLLECTION_NAME: &str = "confirmations";

