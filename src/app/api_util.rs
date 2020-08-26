use super::lib::*;
use bson::Document;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn get_validate_error(e: ValidationErrors) -> Document {
    let mut doc = Document::new();
    for (s, vlk) in e.into_errors().iter() {
        match vlk {
            ValidationErrorsKind::Field(v) => {
                for i in v.iter() {
                    doc.insert(*s, &*i.message.clone().unwrap());
                }
            }
            _ => unimplemented!(),
        };
    }
    doc
}
// pub async fn get_accountId(_req: HttpRequest) -> Option<String> {
//     let _auth = _req.headers().get("Authorization");
//     let _spilt: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//     let token = _spilt[1].trim();
//     match cb_users::check_token(token).await {
//         Some(result) => Some(result.email),
//         None => None,
//     }
// }
// pub async fn check_token(token: &str) -> Option<User> {
//     let _var = &CONFIG.secret_key;
//     let key = _var.as_bytes();
//     let _decode = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(key),
//         &Validation::default(),
//     );
//     match _decode {
//         Ok(decoded) => users_db::find_by_email(decoded.claims.sub.to_string())
//             .await
//             .unwrap(),
//         Err(e) => None,
//     }
// }
// pub async fn check_auth(_req: HttpRequest) -> HttpResponse {
//     let _auth = _req.headers().get("Authorization");
//     let _spilt: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//     let token = _spilt[1].trim();
//     match check_token(token).await {
//         Some(result) => HttpResponse::Ok().json(Response {
//             data: doc! {},
//             status: true,
//             message: "Your token is valid".to_string(),
//         }),
//         None => HttpResponse::Ok().json(Response {
//             data: doc! {},
//             status: true,
//             message: "Your token is invalid".to_string(),
//         }),
//     }
// }
