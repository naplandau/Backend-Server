use bson::{Document};
use chrono::{Utc};
use crate::utils::hasher::HASHER;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: String,
    pub created_by: String,
    pub created_time_dt: i64,
    pub updated_by: String,
    pub updated_time_dt: i64,
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct Login {
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Register {
    #[validate(required, email(message = "email is not valid"))]
    pub email: Option<String>,
    #[validate(
        required,
        length(min = 8, message = "password must be at least 8 characters")
    )]
    pub password: Option<String>,
}
// #[derive(Serialize, Deserialize, Debug, Validate, Clone)]
// pub struct Confirmation {
//     pub id: String,
//     pub email: String,
//     pub password: String,
//     pub expires_time_dt: DateTime,
// }
#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateUser {
    // #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    // #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub role: Option<String>,
    #[validate(phone(message = "phone_number not valid"))]
    // #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
}

lazy_static! {
    pub static ref ADMIN_DOC: Document = bson::doc! {
        "id": "user_06e75640-09b0-4b8f-b06f-a4af47aebd4a".to_string(),
        "email": "admin@gmail.com".to_string(),
        "password": HASHER.hash("123456789").unwrap(),
        "first_name": "ADMIN".to_string(),
        "last_name": "STARTED".to_string(),
        "phone_number": "+84767336687".to_string(),
        "role": "ADMIN".to_string(),
        "created_by": "admin".to_string(),
        "created_time_dt": Utc::now().timestamp(),
        "updated_by": "admin".to_string(),
        "updated_time_dt": Utc::now().timestamp(),
        "status": 1
    };
}
