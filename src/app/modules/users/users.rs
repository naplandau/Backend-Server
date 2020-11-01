use super::lib::*;
use actix_web::FromRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::dev::Payload;
use futures::future::{Ready, ok, err};
use actix_session::Session;

pub async fn create_users(req: web::Json<Register>) -> HttpResponse {
    match req.validate() {
        Ok(_) => {
            let email = req.email.to_owned().unwrap_or_default();
            let find_res = users_db::find_by_email(email.to_owned()).await.unwrap();
            match find_res {
                Some(_) => Error::Conflict.error_response(),
                None => {
                    let user_save: User = req.to_owned().into();
                    match users_db::insert(user_save.to_owned()).await {
                        Ok(_id) => {
                            HttpResponse::Created().json(Response::from(user_save.to_owned()))
                        }
                        Err(e) => Error::from(e).error_response(),
                    }
                }
            }
        }
        Err(e) => Error::from(e).error_response(),
    }
}
pub async fn get_users(_query: web::Query<HashMap<String, String>>, _:Authorized, _session: Session) -> HttpResponse {
    println!("Into function");

    if let Some(count) = _session.get::<i32>("counter").unwrap(){
        _session.set("counter", count+1).unwrap();
        println!("Session: {}",count+1);
    } else {
        _session.set("counter", 1).unwrap();
        println!("Session start!: {}", 1);
    }

    let option = Some(
        FindOptions::builder()
            //.sort(doc! {"title":1})
            .build(),
    );
    let filter = Some(doc! {});
    let data = users_db::find_all(filter, option).await;
    match data {
        Ok(vec) => {
            println!("Go out function");
            HttpResponse::Ok().json(ResponseList {
            data: vec_user_to_vec_docs(vec),
            status: true,
            message: "success".to_string(),
        })},
        Err(_e) => {
            // error!("get_users: {:?}", _e);
            println!("Go out function");
            Error::InternalServerError.error_response()
        }
    }
}
pub async fn get_user(id: web::Path<String>) -> HttpResponse {
    let find_res = users_db::find_by_id(id.to_string()).await.unwrap();
    match find_res {
        Some(user) => HttpResponse::Ok().json(Response::from(user)),
        None => {
            error!("get_user: Not Found");
            Error::NoContent.error_response()
            //Error::NotFound("User Not Found".to_string()).error_response()
        }
    }
}
/// Now return old document before update
pub async fn update_user(req: web::Json<UpdateUser>, id: web::Path<String>) -> HttpResponse {
    match req.validate() {
        Ok(_) => {
            let find_res = users_db::find_by_id(id.to_string()).await.unwrap();
            match find_res {
                Some(user) => {
                    let update_doc = bson::to_document(&req.to_owned()).unwrap();
                    let _execs = users_db::update(user, update_doc).await;
                    match _execs {
                        Ok(user) => HttpResponse::Ok().json(Response {
                                data: get_sub_field(&bson::to_document(&user).unwrap()),
                                message: "Success".to_string(),
                                status: true,
                            }),
                        Err(e) => {
                            println!("{:?}", e);
                            Error::InternalServerError.error_response()
                        }
                    }
                }
                None => Error::NoContent.error_response(),
            }
        }
        Err(e) => {
            println!("Validate error: {:?}", e);
            Error::from(e).error_response()
        }
    }
}
pub async fn delete_user(id: web::Path<String>) -> HttpResponse {
    let data = users_db::find_by_id(id.to_owned()).await.unwrap();
    match data {
        Some(_) => {
            let res = users_db::delete_by_id(id.to_owned()).await;
            match res {
                Ok(op) => match op {
                    Some(u) => HttpResponse::Ok().json(Response::from(u.to_owned())),
                    None => Error::NoContent.error_response(),
                },
                Err(_) => Error::InternalServerError.error_response(),
            }
        }
        None => {
            error!("delete_user: Not Found");
            Error::NoContent.error_response()
        }
    }
}
pub async fn delete_users() -> HttpResponse {
    let res = users_db::delete_all().await;
    match res {
        Ok(deleted) => HttpResponse::Ok().json(doc! {"deleted": deleted}),
        Err(_) => Error::InternalServerError.error_response(),
    }
}
pub async fn find_delete_user(id: web::Path<String>) -> HttpResponse {
    match users_db::find_by_id_and_delete(id.to_owned()).await {
        Ok(op) => match op {
            Some(user) => HttpResponse::Ok().json(Response::from(user)),
            None => Error::NoContent.error_response(),
        },
        Err(_) => Error::InternalServerError.error_response(),
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
                Err(_) => Error::InternalServerError.error_response(),
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

impl From<Register> for User {
    fn from(register: Register) -> Self {
        let current_time = Utc::now();
        User {
            id: String::from("user_") + &Uuid::new_v4().to_simple().to_string(),
            email: register.email.unwrap_or_default().to_owned(),
            password: register.password.unwrap_or_default().to_owned(),
            first_name: None,
            last_name: None,
            phone_number: None,
            role: "USER".to_owned(),
            created_by: "".to_owned(),
            created_time_dt: bson::DateTime(current_time),
            updated_by: "".to_owned(),
            updated_time_dt: bson::DateTime(current_time),
            status: 1,
        }
    }
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
pub struct Authorized;
impl FromRequest for Authorized {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            ok(Authorized)
        } else {
            err(ErrorUnauthorized("not authorized"))
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(value) = req.headers().get("Authorization") {
        // actual implementation that checks header here
        dbg!(value);
        true
    } else {
        false
    }
}