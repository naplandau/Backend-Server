use super::lib::*;

pub async fn delete(user: web::Json<Delete>) -> HttpResponse {
    let user = user.into_inner();
    let data = users_db::find_by_email(user.email.to_string())
        .await
        .unwrap();
    match data {
        Some(_) => HttpResponse::Ok().json(Response {
            data: doc! {},
            message: "delete success".to_string(),
            status: true,
        }),
        None => {
            error!("delete_user: Not Found");
            Error::NotFound("User Not Found".to_string()).error_response()
        }
    }
}
pub async fn create_users(req: web::Json<Register>) -> HttpResponse {
    match req.validate() {
        Ok(_) => {
            let user = users_db::find_by_email(req.email.to_string())
                .await
                .unwrap();
            match user {
                Some(_) => Error::Conflict.error_response(),
                None => match users_db::insert(req.to_owned().into()).await {
                    Ok(id) => {
                        println!("{}",id);
                        HttpResponse::Created().json(req.to_owned())
                    },
                    Err(e) => Error::from(e).error_response()
                }
            }
        },
        Err(e) => Error::from(e).error_response(),
    }
}
pub async fn get_users(_query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let option = FindOptions::builder()
        //.sort(doc! {"title":1})
        .build();
    let data = users_db::find_all(doc! {}, option).await;
    match data {
        Ok(vec) => match vec {
            Some(v) => HttpResponse::Ok().json(ResponseList {
                data: vec_user_to_vec_docs(v),
                status: true,
                message: "success".to_string(),
            }),
            None => {
                error!("get_users: None");
                return Error::InternalServerError.error_response();
            }
        },
        Err(e) => {
            error!("get_users: {:?}", e);
            return Error::InternalServerError.error_response();
        }
    }
}
pub async fn get_user(id: web::Path<String>) -> HttpResponse {
    let user = users_db::find(id.to_string()).await.unwrap();
    match user {
        Some(v) => HttpResponse::Ok().json(Response {
            data: get_sub_field(&prepare_user(v)),
            message: "Success".to_string(),
            status: true,
        }),
        None => {
            error!("get_user: Not Found");
            Error::NotFound("User Not Found".to_string()).error_response()
        }
    }
}
pub async fn update_user(_user: web::Json<Update>, id: web::Path<String>) -> HttpResponse {
    let user = users_db::find(id.to_string()).await.unwrap();
    match user {
        Some(v) => {
            // let _execs = db_utils::insert(PENDING_COLLECTION, &user_doc).await;
            HttpResponse::Ok().json(Response {
                data: get_sub_field(&prepare_user(v)),
                message: "Success".to_string(),
                status: true,
            })
        }
        None => HttpResponse::Ok().json(Response {
            data: doc! {},
            message: "Not Found".to_string(),
            status: false,
        }),
    }
}
pub async fn admin() -> HttpResponse {
    let email = ADMIN_DOC.get_str("email").unwrap();
    let _exists = users_db::find_by_email(email.to_string()).await.unwrap();
    match _exists {
        Some(_) => HttpResponse::Ok().json(Response {
            data: get_sub_field(&*ADMIN_DOC),
            message: "Success".to_string(),
            status: true,
        }),
        None => {
            let _exec = db_utils::insert("users", &ADMIN_DOC).await;
            match _exec {
                Ok(_) => HttpResponse::Ok().json(Response {
                    data: get_sub_field(&*ADMIN_DOC),
                    message: "Success".to_string(),
                    status: true,
                }),
                Err(_) => HttpResponse::Ok().json(Response {
                    data: doc! {},
                    status: false,
                    message: "Something went wrong.".to_string(),
                }),
            }
        }
    }
}
fn vec_user_to_vec_docs(vec: Vec<User>) -> Vec<Document> {
    let mut res: Vec<Document> = Vec::new();
    for user in vec.iter() {
        res.push(get_sub_field(&bson::to_document(&user).unwrap()));
    }
    res
}
fn prepare_user(user: User) -> Document {
    // let current_time = Utc::now();
    doc! {
        "id": user.id.to_string(),
        "email": user.email.to_string(),
        // "password": HASHER.hash(&user.password).unwrap(),
        "first_name": user.first_name.to_string(),
        "last_name": user.last_name.to_string(),
        "phone_number": user.phone_number.to_string(),
        "role": user.role.to_string(),
        "created_by": user.created_by.to_string(),
        "created_time_dt": user.created_time_dt.naive_utc().to_string(),
        "updated_by": user.updated_by.to_string(),
        "updated_time_dt": user.updated_time_dt.naive_utc().to_string(),
        "status": user.status,
    }
}
fn prepare_update(user: User, update_user: Update) -> Document {
    let current_time = Utc::now();
    let mut docs = prepare_user(user);
    let update_doc = doc! {
        "updated_by": update_user.email.to_string(),
        "updated_time_dt": current_time.naive_utc().to_string(),
    };
    docs.extend(update_doc);
    docs
}
impl From<Register> for User {
    fn from(register: Register) -> Self {
        let current_time = Utc::now();
        User {
            id: String::from("user_") + &Uuid::new_v4().to_simple().to_string(),
            email: register.email.to_owned(),
            password: HASHER.hash(&register.password).unwrap(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            phone_number: "".to_string(),
            role: "USER".to_string(),
            created_by: "".to_string(),
            created_time_dt: bson::DateTime(current_time),
            updated_by: "".to_string(),
            updated_time_dt: bson::DateTime(current_time),
            status: 1,
        }
    }
}
