use super::db_utils;
use crate::config::config::CONFIG;
use crate::core::models::{api_response::*, users::*};
use crate::server::handlers::hasher::{HASHER, hash_validation};
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::error::Error;
use uuid::Uuid;
use bson::doc;
use bson::{Document, Bson};

const COLLECTION_NAME: &str = "users";

pub async fn find_user(id: String) -> Result<Option<User>, Error> {
    let cursor = db_utils::find(COLLECTION_NAME, id).await.unwrap();
    match cursor {
        Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
            Ok(model) => Ok(model),
            Err(e) => Err(Error::from(e)),
        },
        None => Ok(None),
    }
}
pub async fn find_user_by_email(email: String) -> Result<Option<User>, Error> {
    let field = doc!{
        "email": email
    };
    let cursor = db_utils::find_by(COLLECTION_NAME, field).await.unwrap();
    match cursor {
        Some(doc) => {
            match bson::from_bson(bson::Bson::Document(doc)) {
            Ok(model) => Ok(model),
            Err(e) => Err(Error::from(e)),
        }},
        None => Ok(None),
    }
}
pub async fn login(user: Login) -> Result<LoginResponse, Response> {
    let data = find_user_by_email(user.email.to_string()).await.unwrap();
    match data {
        Some(x) => {
            if hash_validation(x.password,user.password) {
                let _var = &CONFIG.secret_key;
                let key = _var.as_bytes();

                let mut _date: DateTime<Utc>;
                //Remember me
                if !user.remember_me {
                    _date = Utc::now() + Duration::hours(1);
                } else {
                    _date = Utc::now() + Duration::days(365);
                }
                let my_claims = Claims {
                    sub: user.email,
                    expire_time: _date.timestamp() as usize,
                };
                let token = encode(
                    &Header::default(),
                    &my_claims,
                    &EncodingKey::from_secret(key),
                ).unwrap();
                Ok(LoginResponse{
                    data: "You have successfully logged in.".to_string(),
                    auth_token: token.clone(),
                    status: true,
                    request_id: token.clone()
                })
            }
            else{
                Err(Response{
                    status: false,
                    message: "Check your infomations".to_string(),
                })
            }
        },
        None => Err(Response{
            status: false,
                    message: "Check your infomations".to_string(),
        })
    }
}
pub async fn register(user: Register) -> Response{
    let _exits = find_user_by_email(user.email.to_string()).await.unwrap();
    match _exits {
        Some(_) => {
            Response{
                message:"This account has been exists!".to_string(),
                status: false,
            }
        },
        None =>{
            //let hash_pw = HASHER.hash(&user.password).unwrap();
            //let user_id = String::from("user") + &Uuid::new_v4().to_string();
            let user_doc = prepare_register_user(user);
            let _exec = db_utils::insert(COLLECTION_NAME, &user_doc).await;
            match _exec {
                Ok(_) => Response{
                    status: true,
                    message:"Register successfull.".to_string(),
                },
                Err(_) => Response{
                    status: false,
                    message:"Wrong.".to_string(),
                }

            }
        }
    }
}
fn prepare_register_user(user: Register) -> Document{
    let current_time = Utc::now();
    doc!{
        "id": String::from("user") + &Uuid::new_v4().to_string(),
        "email": user.email.to_string(),
        "password": HASHER.hash(&user.password).unwrap(),
        "first_name": Bson::Null ,
        "last_name": Bson::Null,
        "phone_number": Bson::Null,
        "role": Bson::Null,
        "roles": Bson::Null,
        "avatar": Bson::Null,
        "time_zone": Bson::Null,
        "created_by": Bson::Null,
        "created_time_dt": Bson::DateTime(current_time),
        "updated_by": Bson::Null,
        "updated_time_dt": Bson::DateTime(current_time),
        "status": 0,
        "confirm_code": Bson::Null,
        "confirm_code_created_time_dt": Bson::DateTime(current_time)
    }
}