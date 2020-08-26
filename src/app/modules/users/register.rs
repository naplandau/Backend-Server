use super::lib::*;
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
                            message: "Register successfully.".to_string(),
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
        //"roles": "".to_string(),
        //"avatar":"".to_string(),
        //"time_zone": 7,
        "created_by": "".to_string(),
        "created_time_dt": Bson::DateTime(current_time),
        "updated_by": "".to_string(),
        "updated_time_dt": Bson::DateTime(current_time),
        "status": 0,
    }
}
