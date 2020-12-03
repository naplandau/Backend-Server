use super::super::lib::*;
use crate::core::redis_db::*;
use crate::nats_broker::*;

pub async fn create_users(
    req: web::Json<Register>,
    _pool: web::Data<RedisFactory>,
    nats_pool: web::Data<NatsConnection>,
) -> HttpResponse {
    let req_data = NatsRequest {
        request_type: "".to_owned(),
        request_id: "".to_owned(),
        from: "client".to_owned(),
        data: serde_json::to_value(&req.to_owned()).unwrap(),
        status: true,
        status_code: 0,
        status_des: "success".to_string(),
        send_time: Utc::now().timestamp(),
    };
    let resp = nats_pool.request("user.create", serde_json::to_string(&req_data).unwrap());
    match resp {
        Ok(msg) => {    
            let nats_res = NatsResponse::from(msg.clone());
            match nats_res.status_code {
                0 => HttpResponse::Created().json(Response::from(nats_res)),
                _ => ServerError::InternalServerError.error_response(),
            }
        }
        Err(_e) => ServerError::InternalServerError.error_response(),
    }
}
// pub async fn get_users(query: web::Query<HashMap<String, String>>) -> HttpResponse {
//     match modules::get_users(query.to_owned()).await {
//         Ok(vec) => HttpResponse::Ok().json(ResponseList {
//             data: vec_user_to_vec_docs(vec),
//             status: true,
//             message: "success".to_string(),
//         }),
//         Err(e) => ServerError::from(e).error_response(),
//     }
// }
// pub async fn get_user(id: web::Path<String>) -> HttpResponse {
//     match modules::get_user(id.to_owned()).await {
//         Ok(user) => HttpResponse::Ok().json(Response::from(user)),
//         Err(e) => ServerError::from(e).error_response(),
//     }
// }
// /// Now return old document before update
// pub async fn update_user(req: web::Json<UpdateUser>, id: web::Path<String>) -> HttpResponse {
//     match modules::update_user(req.to_owned(), id.to_owned()).await {
//         Ok(user) => HttpResponse::Ok().json(Response {
//             data: get_sub_field(&bson::to_document(&user).unwrap()),
//             message: "Success".to_string(),
//             status: true,
//         }),
//         Err(e) => ServerError::from(e).error_response(),
//     }
// }
// pub async fn delete_user(id: web::Path<String>) -> HttpResponse {
//     match modules::delete_user(id.to_owned()).await {
//         Ok(user) => HttpResponse::Ok().json(Response::from(user)),
//         Err(e) => ServerError::from(e).error_response(),
//     }
// }
// pub async fn delete_users() -> HttpResponse {
//     match modules::delete_users().await {
//         Ok(count) => HttpResponse::Ok().json(doc! {"deleted": count}),
//         Err(e) => ServerError::from(e).error_response(),
//     }
// }
// pub async fn find_delete_user(id: web::Path<String>) -> HttpResponse {
//     match users_db::find_by_id_and_delete(id.to_owned()).await {
//         Ok(op) => match op {
//             Some(user) => HttpResponse::Ok().json(Response::from(user)),
//             None => ServerError::NoContent.error_response(),
//         },
//         Err(_) => ServerError::InternalServerError.error_response(),
//     }
// }
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
// fn vec_user_to_vec_docs(vec: Vec<User>) -> Vec<Document> {
//     let mut res: Vec<Document> = Vec::new();
//     for user in vec.iter() {
//         res.push(get_sub_field(&bson::to_document(&user).unwrap()));
//     }
//     res
// }
impl From<NatsResponse> for Response {
    fn from(nats_resp: NatsResponse) -> Self {
        Response {
            data: get_sub_field(&serde_json::to_value(&nats_resp.data).unwrap()),
            message: nats_resp.status_des,
            status: nats_resp.status,
        }
    }
}
