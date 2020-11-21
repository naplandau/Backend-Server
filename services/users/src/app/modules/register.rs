use super::super::lib::*;
const PENDING_COLLECTION: &str = "users_pending";
const USERS_COLLECTION: &str = "users";
const TIMEOUT_PENDING: i64 = 1; //Hours
pub async fn register(user: web::Json<Register>) -> HttpResponse {
    match user.validate() {
        Ok(_) => {
            let _exits = users_db::find_by_email(user.email.to_owned().unwrap_or_default())
                .await
                .unwrap();
            match _exits {
                Some(_) => HttpResponse::Ok().json(Response {
                    data: doc! {},
                    message: "This account has been exists!".to_string(),
                    status: false,
                }),
                None => {
                    if CONFIG.dev_mode == true {
                        let user_doc = prepare_user(user.to_owned());
                        let _exec = db_utils::insert(USERS_COLLECTION, &user_doc).await;
                        match _exec {
                            Ok(_) => HttpResponse::Ok()
                                .status(StatusCode::from_u16(201).unwrap())
                                .json(Response {
                                    data: get_sub_field(&user_doc),
                                    status: true,
                                    message: "Register Succcessfull".to_string(),
                                }),
                            Err(_) => HttpResponse::Ok()
                                .status(StatusCode::from_u16(500).unwrap())
                                .json(Response {
                                    data: doc! {},
                                    status: false,
                                    message: "Internal Server Error".to_string(),
                                }),
                        }
                    } else {
                        let _confirm_id = String::from("confirm_") + &Uuid::new_v4().to_string();
                        let _confirm = Confirmation {
                            id: _confirm_id,
                            email: user.email.to_owned().unwrap_or_default(),
                            password: user.password.to_owned().unwrap_or_default(),
                            expires_time_dt: bson::DateTime(
                                Utc::now() + Duration::hours(TIMEOUT_PENDING),
                            ),
                        };
                        let _exec = send_confirmation_mail(&_confirm).await;
                        let user_doc = prepare_pending_user(user.clone(), _confirm.clone());
                        let _execs = db_utils::insert(PENDING_COLLECTION, &user_doc).await;
                        match _execs {
                            Ok(_) => HttpResponse::Ok().json(Response {
                                data: get_sub_field(&user_doc),
                                status: true,
                                message: "Check your mail to complete your Registration"
                                    .to_string(),
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
        }
        Err(e) => ServerError::from(e).error_response(),
    }
}
pub async fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), ()> {
    let domain_url = std::env::var("DOMAIN").expect("DOMAIN must be set");
    let expires = confirmation
        .expires_time_dt
        .with_timezone(&Local)
        .format("%I:%M %p %A, %-d %B, %C%y")
        .to_string();
    let html_text = format!(
        "Please click on the link below to complete registration. <br/>
       <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
      This link expires on <strong>{expires}</strong>",
        domain = domain_url,
        id = confirmation.id,
        expires = expires
    );
    let _plain_text = format!(
        "Please visit the link below to complete registration:\n
      {domain}/register/{id}\n
      This link expires on {expires}.",
        domain = domain_url,
        id = confirmation.id,
        expires = expires
    );
    Ok(())
    // let email = Email::builder()
    //     .to(confirmation.email.clone())
    //     .from(("noreply@auth-started.com", "STARTED"))
    //     .subject("Complete your registration on our one-of-a-kind Auth Service")
    //     //.text(plain_text)
    //     .html(html_text)
    //     .build()
    //     .unwrap();
    //
    // let result = send_email(email);
    // match result {
    //     Ok(v) => {
    //         println!("Response: {:#?}", v);
    //         Ok(())
    //     }
    //     Err(e) => {
    //         println!("Error: {:#?}", e);
    //         Ok(())
    //     }
    // }
}

pub async fn verify_register(id: web::Path<String>) -> HttpResponse {
    let data = users_db::find_pending(id.to_string()).await.unwrap();
    match data {
        Some(doc) => {
            let user_doc = prepare_register_user(doc.clone());
            // let _delete_pending =
            //     db_utils::delete_filter(PENDING_COLLECTION, doc! {"email": doc.email})
            //         .await
            //         .unwrap();

            let _exec = db_utils::insert(USERS_COLLECTION, &user_doc).await;
            match _exec {
                Ok(_) => HttpResponse::Ok()
                    .status(StatusCode::from_u16(201).unwrap())
                    .json(Response {
                        data: get_sub_field(&user_doc),
                        status: true,
                        message: "Register Succcessfull".to_string(),
                    }),
                Err(_) => HttpResponse::Ok()
                    .status(StatusCode::from_u16(500).unwrap())
                    .json(Response {
                        data: doc! {},
                        status: false,
                        message: "Internal Server Error".to_string(),
                    }),
            }
        }
        None => HttpResponse::Ok()
            .status(StatusCode::from_u16(401).unwrap())
            .json(Response {
                data: doc! {},
                status: false,
                message: "Check your information".to_string(),
            }),
    }
}
fn prepare_pending_user(user: Register, confirm: Confirmation) -> Document {
    let current_time = Utc::now();
    doc! {
        "id": confirm.id,
        "email": user.email.unwrap_or_default().to_string(),
        "password": HASHER.hash(&user.password.unwrap_or_default()).unwrap(),
        "created_by": "CUSTOMER".to_string(),
        "created_time_dt": Bson::DateTime(current_time),
        "expires_time_dt": Bson::DateTime(*confirm.expires_time_dt),
    }
}
fn prepare_register_user(user: Confirmation) -> Document {
    let current_time = Utc::now();
    doc! {
        "id": String::from("user_") + &Uuid::new_v4().to_string(),
        "email": user.email.to_string(),
        "password": user.password,
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
fn prepare_user(user: Register) -> Document {
    let current_time = Utc::now();

    doc! {
        "id": String::from("user_") + &Uuid::new_v4().to_string(),
        "email": user.email.unwrap_or_default().to_string(),
        "password": HASHER.hash(&user.password.unwrap_or_default()).unwrap(),
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
