use actix_web::web;
use actix_web::{HttpResponse, Responder};
use futures::StreamExt;
// use actix_session::{CookieSession, Session};

#[derive(Debug, Deserialize, Serialize)]
struct KL {
    a: Option<String>,
    b: Option<String>,
}
async fn index1(query: web::Json<serde_json::Value>) -> HttpResponse {
    println!("{:#?}", query.get("a"));
    let obj = query.get("a");
    match obj {
        Some(v) => {
            let b = v.get("b");
            let c = v.get("c");
            println!("b:{:?}, c:{:?}", b, c);
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("API ARTICLE POST")
}

async fn index(mut body: web::Payload) -> HttpResponse {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    format!("Body {:?}!", bytes);
    HttpResponse::Ok().body(bytes)
}

async fn index3(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("API ARTICLE GET DETAIL".to_owned() + &*id.to_string())
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Test2 {
    // #[serde(skip_des = "Option::is_none")]
    // #[serde(default = ("kl"))]
    pub x: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<i32>,
}
async fn test1(data: web::Json<Test2>) -> HttpResponse {
    print!("test1");
    println!("{:?}", data);
    HttpResponse::Ok().finish()
}

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("articles")
            //.route(web::get().to(index1))
            .route(web::post().to(test1)),
    )
    .service(web::resource("articles/{id}").route(web::get().to(test1)));
}
