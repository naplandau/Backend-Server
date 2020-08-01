use actix_web::{
    error::{ResponseError},
    http::StatusCode,
    HttpResponse
};
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum Error{
    BadRequest(String),
    BlockingError(String),
    //404
    NotFound(String),
    //401
    Unauthorized(String),
    //403
    Forbidden(String),
    InternalServerError
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse{
    errors: Vec<String>
}
impl From<&String> for ErrorResponse{
    fn from(error: &String) -> Self{
        ErrorResponse{
            errors: vec![error.into()]
        }
    }
}
impl From<Vec<String>> for ErrorResponse{
    fn from(errors: Vec<String>) -> Self{
        ErrorResponse{errors}
    }
}
impl ResponseError for Error{
    fn error_response(&self) -> HttpResponse{
        match self{
            Error::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            Error::NotFound(error) =>{
                HttpResponse::NotFound().json::<ErrorResponse>(error.into())
            }
            // Error::Unauthorized(error) =>{
            //     HttpResponse::Unauthorized.json::<ErrorResponse>(error.into())
            // }
            // Error::Forbidden(error) =>{
            //     HttpResponse::Forbidden.json::<ErrorResponse>(error.into())
            // }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}