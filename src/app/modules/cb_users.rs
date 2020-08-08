use crate::core::db::db_utils;
use crate::core::db::{users_db::*, AppState, PgPool};
use crate::core::models::users::{User};
use crate::server::handlers::HASHER;
use crate::server::handlers::{
    errors::Error,
    helpers::{respond_json, respond_ok},
};
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use actix_web::Responder;
use bson::doc;
use chrono::{NaiveDateTime, Utc};
use futures::stream::StreamExt;
use mongodb::Client;
use rayon::prelude::*;
use std::sync::Mutex;
use uuid::Uuid;
use validator::Validate;

// #[derive(Debug, Deserialize)]
// pub struct In<U> {
//     user: U,
// }

// #[derive(Debug, Deserialize, Serialize, PartialEq)]
// pub struct UserResponse {
//     pub id: String,
//     pub username: String,
//     pub email: Option<String>,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub phone_number: Option<String>,
//     pub status: Option<i16>,
//     pub role: Option<String>,
//     pub created_time_dt: NaiveDateTime,
//     pub updated_time_dt: NaiveDateTime,
// }
// impl From<User> for UserResponse {
//     fn from(user: User) -> Self {
//         UserResponse {
//             id: user.id,
//             username: user.username,
//             email: user.email,
//             first_name: user.first_name,
//             last_name: user.last_name,
//             phone_number: user.phone_number,
//             status: Some(user.status),
//             role: user.role,
//             created_time_dt: user.created_time_dt,
//             updated_time_dt: user.updated_time_dt,
//         }
//     }
// }
// impl From<NewUser> for User {
//     fn from(new_user: NewUser) -> Self {
//         User {
//             id: new_user.id,
//             username: new_user.username,
//             email: new_user.email,
//             first_name: new_user.first_name,
//             last_name: new_user.last_name,
//             phone_number: new_user.phone_number,
//             password: HASHER.hash(&new_user.password).unwrap(),
//             role: Option::None,
//             roles: Option::None,
//             avatar: Option::None,
//             time_zone: Option::None,
//             created_by: Option::None,
//             created_time_dt: Utc::now().naive_utc(),
//             updated_by: Option::None,
//             updated_time_dt: Utc::now().naive_utc(),
//             status: 0,
//             confirm_code: Option::None,
//             confirm_code_created_time_dt: Utc::now().naive_utc(),
//         }
//     }
// }
// impl From<UpdateUser> for User {
//     fn from(new_user: UpdateUser) -> Self {
//         User {
//             id: new_user.id,
//             username: new_user.username,
//             email: new_user.email,
//             first_name: new_user.first_name,
//             last_name: new_user.last_name,
//             phone_number: new_user.phone_number,
//             password: String::from("null"),
//             role: Option::None,
//             roles: Option::None,
//             avatar: Option::None,
//             time_zone: Option::None,
//             created_by: Option::None,
//             created_time_dt: Utc::now().naive_utc(),
//             updated_by: Option::None,
//             updated_time_dt: Utc::now().naive_utc(),
//             status: 0,
//             confirm_code: Option::None,
//             confirm_code_created_time_dt: Utc::now().naive_utc(),
//         }
//     }
// }
// impl From<Vec<User>> for UsersResponse {
//     fn from(users: Vec<User>) -> Self {
//         UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
//     }
// }

// #[derive(Debug, Deserialize, Serialize, PartialEq)]
// pub struct UsersResponse(pub Vec<UserResponse>);

// #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
// pub struct RegisterUser {
//     #[validate(length(
//         min = 3,
//         message = "first_name is required and must be at least 3 characters"
//     ))]
//     pub first_name: String,

//     #[validate(length(
//         min = 3,
//         message = "last_name is required and must be at least 3 characters"
//     ))]
//     pub last_name: String,

//     #[validate(email(message = "email must be a valid email"))]
//     pub email: String,

//     #[validate(length(
//         min = 6,
//         message = "password is required and must be at least 6 characters"
//     ))]
//     pub password: String,

//     #[validate(length(
//         min = 6,
//         max = 20,
//         message = "user_name is required and must be at least 6 characters"
//     ))]
//     pub user_name: String,

//     #[validate(length(
//         min = 9,
//         max = 13,
//         message = "phone_number is required and must be at least 6 characters"
//     ))]
//     pub phone_number: String,
// }

// #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
// pub struct UpdateUserData {
//     #[validate(length(
//         min = 3,
//         message = "first_name is required and must be at least 3 characters"
//     ))]
//     pub first_name: String,

//     #[validate(length(
//         min = 3,
//         message = "last_name is required and must be at least 3 characters"
//     ))]
//     pub last_name: String,

//     #[validate(email(message = "email must be a valid email"))]
//     pub email: String,

//     #[validate(length(
//         min = 6,
//         message = "password is required and must be at least 6 characters"
//     ))]
//     pub password: String,

//     #[validate(length(
//         min = 6,
//         max = 20,
//         message = "user_name is required and must be at least 6 characters"
//     ))]
//     pub user_name: String,

//     #[validate(length(
//         min = 9,
//         max = 13,
//         message = "phone_number is required and must be at least 6 characters"
//     ))]
//     pub phone_number: String,
//     pub role: String,
//     pub roles: String,
//     pub avatar: String,
//     pub time_zone: String,
//     pub updated_by: String,
// }

// /// Get a user
// pub async fn get_user(user_id: String, pool: Data<PgPool>) -> Result<Json<UserResponse>, Error> {
//     let user = block(move || find(&pool, user_id)).await.unwrap();
//     respond_json(user)
// }

// /// Get all users
// pub async fn get_users(pool: Data<PgPool>) -> Result<Json<UsersResponse>, Error> {
//     let users = block(move || find_all(&pool)).await.unwrap();
//     respond_json(users)
// }

// /// Create a user
// pub async fn create_user(
//     pool: Data<PgPool>,
//     params: Json<RegisterUser>,
// ) -> Result<Json<UserResponse>, Error> {
//     let params_user = params.into_inner();
//     params_user.validate();

//     // temporarily use the new user's id for created_at/updated_at
//     // update when auth is added

//     let user_id = String::from("user") + &Uuid::new_v4().to_string();
//     //let user_id = "suser".to_owned() + uid.clone();
//     let new_user: User = NewUser {
//         id: user_id.to_string(),
//         username: params_user.user_name.to_string(),
//         first_name: Some(params_user.first_name.to_string()),
//         last_name: Some(params_user.last_name.to_string()),
//         email: Some(params_user.email.to_string()),
//         password: params_user.password.to_string(),
//         phone_number: Some(params_user.phone_number.to_string()),
//     }
//     .into();
//     let user = block(move || create(&pool, &new_user)).await.unwrap();
//     respond_json(user.into())
// }

// /// Update a user
// pub async fn update_user(
//     user_id: Path<Uuid>,
//     pool: Data<PgPool>,
//     params: Json<UpdateUserData>,
// ) -> Result<Json<UserResponse>, Error> {
//     let params_user = params.into_inner();
//     params_user.validate();

//     // temporarily use the user's id for updated_at
//     // update when auth is added
//     let user_id = Uuid::new_v4();
//     let update_user: User = UpdateUser {
//         id: user_id.to_string(),
//         username: params_user.user_name.to_string(),
//         first_name: Some(params_user.first_name.to_string()),
//         last_name: Some(params_user.last_name.to_string()),
//         email: Some(params_user.email.to_string()),
//         phone_number: Some(params_user.phone_number.to_string()),
//         role: Some(params_user.role),
//         roles: Some(params_user.roles),
//         avatar: Some(params_user.avatar),
//         time_zone: Some(params_user.time_zone),
//         updated_by: Some(params_user.updated_by),
//     }
//     .into();
//     let user = block(move || update(&pool, &update_user)).await.unwrap();
//     respond_json(user.into())
// }

// /// Delete a user
// pub async fn delete_user(user_id: Path<Uuid>, pool: Data<PgPool>) -> Result<HttpResponse, Error> {
//     block(move || delete(&pool, *user_id)).await.unwrap();
//     respond_ok()
// }

pub async fn test_mongo() -> impl Responder {
    //let client = data.lock().unwrap();
    let mut cursor = db_utils::find_all("users").await.unwrap();
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}
