use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use futures::future::{err, ok, Ready};
use actix_web::{dev, Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use crate::core::models::users::Claims;
use crate::config::config::CONFIG;
pub struct AuthorizationService;
impl FromRequest for AuthorizationService{
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();
    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future{
        let auth = req.headers().get("Authorization");
        match auth {
            Some(_) => {
                let _split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let _token = _split[1].trim();
                let _var = &CONFIG.secret_key;
                let key = _var.as_bytes();
                match decode::<Claims>(
                    _token, &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => ok(AuthorizationService),
                    Err(_err) => err(ErrorUnauthorized("invalid token!")),
                }
            },
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}
fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(value) = req.headers().get("authorized") {
        // actual implementation that checks header here
        dbg!(value);
        true
    } else {
        false
    }
}