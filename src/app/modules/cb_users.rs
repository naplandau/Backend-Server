use super::super::lib::*;
use crate::core::models::users::{
    Claims, Delete, Login, Register, Update, UserResponse, ADMIN_DOC,
};
use std::error::Error;
use validator::{Validate};

pub async fn admin() -> impl Responder {
    let _exec = db_utils::insert("users", &ADMIN_DOC).await;
    match _exec {
        Ok(doc) => HttpResponse::Ok().json(Response {
            data: get_sub_field(&*ADMIN_DOC),
            message: "success".to_string(),
            status: true,
        }),
        Err(_) => HttpResponse::Ok().json(Response {
            data: doc! {},
            status: false,
            message: "Wrong.".to_string(),
        }),
    }
}
pub async fn test_mongo() -> impl Responder {
    let mut cursor = db_utils::find_all("users").await.unwrap();
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let mut doc = document.clone();
                doc.remove("_id");
                results.push(get_sub_field(&doc));
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}
pub async fn delete(user: web::Json<Delete>) -> HttpResponse {
    //Result<LoginResponse, Response> {
    let user = user.into_inner();
    let data = users_db::find_by_email(user.email.to_string())
        .await
        .unwrap();
    match data {
        Some(x) => HttpResponse::Ok().json(Response {
            data: doc! {},
            message: "delete success".to_string(),
            status: true,
        }),
        None => HttpResponse::Ok()
            .status(StatusCode::from_u16(401).unwrap())
            .json(Response {
                data: doc! {},
                status: false,
                message: "Check your infomations".to_string(),
            }),
    }
}
pub async fn login(user: web::Json<Login>) -> HttpResponse {
    let user = user.into_inner();
    let data = users_db::find_by_email(user.email.to_string())
        .await
        .unwrap();
    match data {
        Some(x) => {
            if hash_validation(x.password, user.password) {
                let _var = &CONFIG.secret_key;
                let key = _var.as_bytes();

                let mut _date: DateTime<Utc>;
                //Remember me
                if !user.remember_me {
                    _date = chrono::Utc::now() + Duration::hours(1);
                } else {
                    _date = chrono::Utc::now() + Duration::days(365);
                }
                let my_claims = Claims {
                    sub: user.email,
                    exp: _date.timestamp() as usize,
                };
                let token = encode(
                    &Header::default(),
                    &my_claims,
                    &EncodingKey::from_secret(key),
                )
                .unwrap();

                HttpResponse::Ok().json(Response {
                    data: doc! {"auth_token": token.to_string()},
                    message: "".to_string(),
                    status: true,
                })
            } else {
                HttpResponse::Ok()
                    .status(StatusCode::from_u16(401).unwrap())
                    .json(Response {
                        data: doc! {},
                        status: false,
                        message: "Check your infomations".to_string(),
                    })
            }
        }
        None => HttpResponse::Ok()
            .status(StatusCode::from_u16(401).unwrap())
            .json(Response {
                data: doc! {},
                status: false,
                message: "Check your infomations".to_string(),
            }),
    }
}
pub async fn register(user: web::Json<Register>) -> HttpResponse {
    match user.validate() {
        Ok(_) => {
            let user = user.into_inner();
            let _exits = users_db::find_by_email(user.email.to_string())
                .await
                .unwrap();
            match _exits {
                Some(_) => HttpResponse::Ok().json(Response {
                    data: doc! {},
                    message: "This account has been exists!".to_string(),
                    status: false,
                }),
                None => {
                    let user_doc = prepare_register_user(user);
                    let _exec = db_utils::insert("users", &user_doc).await;
                    match _exec {
                        Ok(_) => HttpResponse::Ok().json(Response {
                            data: get_sub_field(&user_doc),
                            status: true,
                            message: "Register successfull.".to_string(),
                        }),
                        Err(_) => HttpResponse::Ok().json(Response {
                            data: doc! {},
                            status: false,
                            message: "Something was wrong.".to_string(),
                        }),
                    }
                }
            }
        }
        Err(e) => {
            let err_doc = api_util::get_validate_error(e);
            HttpResponse::Ok().json(Response {
                data: doc! {"error": Bson::Document(err_doc)},
                status: false,
                message: "Data not valid.".to_string(),
            })
        }
    }
}

fn prepare_register_user(user: Register) -> Document {
    let current_time = Utc::now();
    doc! {
        "id": String::from("user_") + &Uuid::new_v4().to_string(),
        "email": user.email.to_string(),
        "password": HASHER.hash(&user.password).unwrap(),
        "first_name": "".to_string(),
        "last_name": "".to_string(),
        "phone_number": "".to_string(),
        "role": "USER".to_string(),
        "roles": "".to_string(),
        "avatar":"".to_string(),
        "time_zone": 7,
        "created_by": "".to_string(),
        "created_time_dt": Bson::DateTime(current_time),
        "updated_by": "".to_string(),
        "updated_time_dt": Bson::DateTime(current_time),
        "status": 0,
        "confirm_code": "".to_string(),
        "confirm_code_created_time_dt": Bson::DateTime(current_time)
    }
}

fn get_sub_field(doc: &Document) -> Document {
    let mut new_doc = doc.clone();
    let keys = vec![
        "password",
        "time_zone",
        "confirm_code",
        "confirm_code_created_time_dt",
        "created_by",
        "created_time_dt",
        "updated_by",
        "updated_time_dt",
    ];
    for key in keys.iter() {
        new_doc.remove(key);
    }
    new_doc
}

pub async fn check_token(token: &str) -> Option<User> {
    let _var = &CONFIG.secret_key;
    let key = _var.as_bytes();
    let _decode = decode::<Claims>(
        token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    );
    match _decode {
        Ok(decoded) => Some(users_db::find_by_email(decoded.claims.sub.to_string()).await.unwrap().unwrap()),
        Err(e) => None
    }
}
pub async fn check_auth(_req: HttpRequest) -> HttpResponse {
    let _auth = _req.headers().get("Authorization");
    let _spilt: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = _spilt[1].trim();
    match check_token(token).await {
        Some(result) => HttpResponse::Ok().json(Response {
            data: doc! {},
            status: true,
            message: "Your token is valid".to_string(),
        }),
        None => HttpResponse::Ok().json(Response {
            data: doc! {},
            status: true,
            message: "Your token is invalid".to_string(),
        }),
    }
}
