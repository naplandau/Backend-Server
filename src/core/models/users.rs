use crate::core::db::db_utils;
use crate::server::handlers::HASHER;
use bson::{doc, Bson, DateTime, Document};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
// use validator::Validate;
// use validator_derive::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub role: String,
    pub roles: String,
    pub avatar: String,
    pub time_zone: i32,
    pub created_by: String,
    pub created_time_dt: DateTime,
    pub updated_by: String,
    pub updated_time_dt: DateTime,
    pub status: i32,
    pub confirm_code: String,
    pub confirm_code_created_time_dt: DateTime,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub status: i8,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Login {
    //pub user_name: String,
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub expire_time: usize,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Register {
     #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}
pub struct Update {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub roles: Option<String>,
    pub avatar: Option<String>,
    pub time_zone: Option<i8>,
}
pub struct Delete {
    pub email: String,
}
pub struct Auth {
    pub token: String,
}
lazy_static! {
    pub static ref ADMIN_DOC: Document = doc! {
        "id": String::from("user_") + &Uuid::new_v4().to_string(),
        "email": "admin@gmail.com".to_string(),
        "password": HASHER.hash("123456789").unwrap(),
        "first_name": "Thong".to_string(),
        "last_name": "Nguyen".to_string(),
        "phone_number": "+84767336687".to_string(),
        "role": "USER".to_string(),
        "roles": "ADMIN".to_string(),
        "avatar": Bson::Null,
        "time_zone": 7,
        "created_by": "admin".to_string(),
        "created_time_dt": Utc::now(),
        "updated_by": "admin".to_string(),
        "updated_time_dt": Utc::now(),
        "status": 1,
        "confirm_code": "admin".to_string(),
        "confirm_code_created_time_dt": Utc::now(),
    };
}
