use super::super::lib::*;
pub async fn login(user: web::Json<Login>) -> HttpResponse {
    let user = user.into_inner();
    let data = users_db::find_by_email(user.email.to_string())
        .await
        .unwrap();
    match data {
        Some(x) => {
            if hash_validation(x.password, user.password) {
                let mut _date: DateTime<Utc>;
                //Remember me
                if !user.remember_me {
                    _date = chrono::Utc::now() + Duration::hours(1);
                } else {
                    _date = chrono::Utc::now() + Duration::days(365);
                }
                let _my_claims = Claims {
                    sub: user.email,
                    exp: _date.timestamp() as usize,
                };
                // let token = generate_jwt(my_claims);
                let token = "1";
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
                        message: "Check your information".to_string(),
                    })
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
