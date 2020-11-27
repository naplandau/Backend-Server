use super::super::lib::*;
use super::super::modules;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::FromRequest;
use futures::future::{err, ok, Ready};
pub async fn create_users(req: web::Json<Register>) -> HttpResponse {
    match modules::create_users(req.to_owned()).await {
        Ok(user) => HttpResponse::Created().json(Response::from(user.to_owned())),
        Err(e) => ServerError::from(e).error_response(),
    }
}
pub async fn get_users(
    query: web::Query<HashMap<String, String>>,
    _: UserAuthorized,
) -> HttpResponse {
    match modules::get_users(query.to_owned()).await {
        Ok(vec) => HttpResponse::Ok().json(ResponseList {
            data: vec_user_to_vec_docs(vec),
            status: true,
            message: "success".to_string(),
        }),
        Err(e) => ServerError::from(e).error_response(),
    }
}
pub async fn get_user(id: web::Path<String>) -> HttpResponse {
    match modules::get_user(id.to_owned()).await {
        Ok(user) => HttpResponse::Ok().json(Response::from(user)),
        Err(e) => ServerError::from(e).error_response(),
    }
}
/// Now return old document before update
pub async fn update_user(req: web::Json<UpdateUser>, id: web::Path<String>) -> HttpResponse {
    match modules::update_user(req.to_owned(), id.to_owned()).await {
        Ok(user) => HttpResponse::Ok().json(Response {
            data: get_sub_field(&bson::to_document(&user).unwrap()),
            message: "Success".to_string(),
            status: true,
        }),
        Err(e) => ServerError::from(e).error_response(),
    }
}
pub async fn delete_user(id: web::Path<String>) -> HttpResponse {
    match modules::delete_user(id.to_owned()).await {
        Ok(user) => HttpResponse::Ok().json(Response::from(user)),
        Err(e) => ServerError::from(e).error_response(),
    }
}
pub async fn delete_users() -> HttpResponse {
    match modules::delete_users().await {
        Ok(count) => HttpResponse::Ok().json(doc! {"deleted": count}),
        Err(e) => ServerError::from(e).error_response(),
    }
}
pub async fn find_delete_user(id: web::Path<String>) -> HttpResponse {
    match users_db::find_by_id_and_delete(id.to_owned()).await {
        Ok(op) => match op {
            Some(user) => HttpResponse::Ok().json(Response::from(user)),
            None => ServerError::NoContent.error_response(),
        },
        Err(_) => ServerError::InternalServerError.error_response(),
    }
}
pub async fn admin() -> HttpResponse {
    let id = ADMIN_DOC.get_str("id").unwrap();
    let _exists = users_db::find_by_id(id.to_owned()).await.unwrap();
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
                Err(_) => ServerError::InternalServerError.error_response(),
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

impl From<User> for Response {
    fn from(user: User) -> Self {
        Response {
            data: get_sub_field(&bson::to_document(&user).unwrap()),
            message: "success".to_string(),
            status: true,
        }
    }
}
pub struct UserAuthorized;
impl FromRequest for UserAuthorized {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            ok(UserAuthorized)
        } else {
            err(ErrorUnauthorized("not authorized"))
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(value) = req.headers().get("Authorization") {
        // actual implementation that checks header here
        println!("Authorization key: {:?}", value);
        true
    } else {
        false
    }
}
