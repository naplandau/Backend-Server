use actix_web::{
    error::{ResponseError},
    http::StatusCode,
    HttpResponse
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};

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
    PoolError(String),
    InternalServerError(String)
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

/// Convert DBErrors to ApiErrors
impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::InternalServerError("Unknown database error".into())
            }
            _ => Error::InternalServerError("Unknown database error".into()),
        }
    }
}
/// Convert PoolErrors to ApiErrors
impl From<PoolError> for Error {
    fn from(error: PoolError) -> Error {
        Error::PoolError(error.to_string())
    }
}