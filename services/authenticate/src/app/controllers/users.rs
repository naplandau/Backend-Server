use super::super::lib::*;
use super::super::modules;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::FromRequest;
use futures::future::{err, ok, Ready};
pub async fn login(req: web::Json<Register>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn get_users(
    query: web::Query<HashMap<String, String>>,
    _: UserAuthorized,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn get_user(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
/// Now return old document before update
pub async fn update_user(req: web::Json<UpdateUser>, id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn delete_user(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn delete_users() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// pub async fn admin() -> HttpResponse {
//     let id = ADMIN_DOC.get_str("id").unwrap();
//     let _exists = users_db::find_by_id(id.to_owned()).await.unwrap();
//     match _exists {
//         Some(_) => HttpResponse::Ok().json(Response {
//             data: get_sub_field(&*ADMIN_DOC),
//             message: "Success".to_string(),
//             status: true,
//         }),
//         None => {
//             let _exec = db_utils::insert("users", &ADMIN_DOC).await;
//             match _exec {
//                 Ok(_) => HttpResponse::Ok().json(Response {
//                     data: get_sub_field(&*ADMIN_DOC),
//                     message: "Success".to_string(),
//                     status: true,
//                 }),
//                 Err(_) => ServerError::InternalServerError.error_response(),
//             }
//         }
//     }
// }
fn vec_user_to_vec_docs(vec: Vec<User>) -> Vec<serde_json::Value> {
    let mut res: Vec<serde_json::Value> = Vec::new();
    for user in vec.iter() {
        res.push(get_sub_field(&serde_json::to_value(&user).unwrap()));
    }
    res
}

impl From<User> for Response {
    fn from(user: User) -> Self {
        Response {
            data: get_sub_field(&serde_json::to_value(&user).unwrap()),
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
