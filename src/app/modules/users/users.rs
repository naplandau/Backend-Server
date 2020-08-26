use super::lib::*;
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
pub async fn get_all_users() -> impl Responder {
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
pub async fn admin() -> impl Responder {
    let email = ADMIN_DOC.get_str("email").unwrap();
    let _exists = users_db::find_by_email(email.to_string()).await.unwrap();
    match _exists {
        Some(v) => HttpResponse::Ok().json(Response {
            data: get_sub_field(&*ADMIN_DOC),
            message: "success".to_string(),
            status: true,
        }),
        None => {
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
                    message: "Something went wrong.".to_string(),
                }),
            }
        }
    }
}
