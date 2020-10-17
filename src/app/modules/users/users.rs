use super::lib::*;
#[allow(dead_code)]
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
        None => HttpResponse::Ok()
            .status(StatusCode::from_u16(401).unwrap())
            .json(Response {
                data: doc! {},
                status: false,
                message: "Check your infomations".to_string(),
            }),
    }
}
pub async fn get_users() -> HttpResponse {
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
    HttpResponse::Ok().json(ResponseList {
        data: results,
        status: true,
        message: "success".to_string(),
    })
}
pub async fn get_user(id: web::Path<String>) -> HttpResponse {
    let user = users_db::find(id.to_string()).await.unwrap();
    match user {
        Some(v) => HttpResponse::Ok().json(Response {
            data: get_sub_field(&prepare_user(v)),
            message: "Success".to_string(),
            status: true,
        }),
        None => HttpResponse::Ok().json(Response {
            data: doc! {},
            message: "Not Found".to_string(),
            status: false,
        }),
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
        })},
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
        //"roles": "".to_string(),
        //"avatar":"".to_string(),
        //"time_zone": 7,
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
