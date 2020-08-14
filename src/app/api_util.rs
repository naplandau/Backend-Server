use bson::Document;
use validator::{ValidationErrors, ValidationErrorsKind};
use super::lib::*;
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
pub async fn get_accountId(_req: HttpRequest) -> Option<String>{
    let _auth = _req.headers().get("Authorization");
    let _spilt: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = _spilt[1].trim();
    match cb_users::check_token(token).await {
        Some(result) => Some(result.email),
        None => None
    }
}