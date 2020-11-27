use actix_web::{error, error::ResponseError, HttpRequest, HttpResponse};
use failure::Fail;
use mongodb::error::Error as MongoError;
use serde_json::{Map as JsonMap, Value as JsonValue};
use validator::ValidationErrors;

#[derive(Debug, failure::Fail, PartialEq)]
pub enum ServerError {
    #[fail(display = "No content")]
    NoContent,
    #[fail(display = "Bad Request")]
    BadRequest(String),
    #[fail(display = "Blocking Error")]
    BlockingError(String),
    #[fail(display = "Not Found")]
    NotFound(String),
    #[fail(display = "Unauthorized")]
    Unauthorized(String),
    #[fail(display = "Forbidden")]
    Forbidden(String),
    #[fail(display = "Pool Error")]
    PoolError(String),
    #[fail(display = "Internal Server Error")]
    InternalServerError,
    #[fail(display = "Method Not Allowed")]
    MethodNotAllowed,
    #[fail(display = "Bad Gateway")]
    BadGateway,
    #[fail(display = "Resource Exists")]
    Conflict,
    #[fail(display = "Database Error")]
    DBError(String),
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),
    #[fail(display = "Time Out")]
    RequestTimeOut,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResErr {
    errors: Vec<String>,
}

impl From<&String> for ResErr {
    fn from(error: &String) -> Self {
        ResErr {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ResErr {
    fn from(errors: Vec<String>) -> Self {
        ResErr { errors }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::UnprocessableEntity(json) => HttpResponse::BadRequest().json(json),
            ServerError::NotFound(err) => HttpResponse::NotFound().json::<ResErr>(err.into()),
            ServerError::Unauthorized(err) => {
                HttpResponse::Unauthorized().json::<ResErr>(err.into())
            }
            ServerError::Forbidden(err) => HttpResponse::Forbidden().json::<ResErr>(err.into()),
            ServerError::RequestTimeOut => HttpResponse::RequestTimeout().finish(),
            ServerError::Conflict => HttpResponse::Conflict().finish(),
            ServerError::BadGateway => HttpResponse::BadGateway().finish(),
            ServerError::NoContent => HttpResponse::NoContent().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl From<ValidationErrors> for ServerError {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors.iter().map(|error| json!(error.message)).collect();
            err_map.insert(field.to_string(), json!(errors));
        }
        ServerError::UnprocessableEntity(json!({
            "errors": err_map,
        }))
    }
}

impl From<MongoError> for ServerError {
    fn from(error: MongoError) -> Self {
        ServerError::DBError(error.to_string())
    }
}
