use actix_web::{
    error::{ResponseError},
    http::StatusCode,
    HttpResponse
};
use failure::Fail;
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};

#[derive(Debug, Fail, PartialEq)]
#[allow(dead_code)]
pub enum Error{
    #[fail(display = "Bad Request")]
    BadRequest(String),
    #[fail(display = "Blocking Error")]
    BlockingError(String),
    //404
    #[fail(display = "Not Found")]
    NotFound(String),
    //401
    #[fail(display = "Unauthorized")]
    Unauthorized(String),
    //403
    #[fail(display = "Forbidden")]
    Forbidden(String),
    #[fail(display = "Pool Error")]
    PoolError(String),
    #[fail(display = "Internal Server Error")]
    InternalServerError(String),
    #[fail(display = "Time Out")]
    TimeOut
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
    fn status_code(&self) -> StatusCode{
        match *self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::GATEWAY_TIMEOUT
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