use uuid::Uuid;
use validator::Validate;
use crate::core::models::users::{NewUser, UpdateUser, User};
use chrono::NaiveDateTime;
use rayon::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub status: Option<i16>,
    pub role: Option<String>,
    pub created_time_dt: NaiveDateTime,
    pub updated_time_dt: NaiveDateTime
}
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number,
            status: user.status,
            role: user.role,
            created_time_dt: user.created_time_dt,
            updated_time_dt: user.updated_time_dt
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UsersResponse(pub Vec<UserResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(length(
        min = 3,
        message = "first_name is required and must be at least 3 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 3,
        message = "last_name is required and must be at least 3 characters"
    ))]
    pub last_name: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

