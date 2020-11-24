use actix_web::{error, error::ResponseError, HttpRequest, HttpResponse};

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
pub struct ErrorResponse {
    errors: Vec<String>,
}
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}
impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::BadRequest(err) => HttpResponse::BadRequest().json::<ErrorResponse>(err.into()),

            ServerError::NotFound(err) => HttpResponse::NotFound().json::<ErrorResponse>(err.into()),
            ServerError::Unauthorized(err) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(err.into())
            }
            ServerError::Conflict => HttpResponse::Conflict().finish(),
            ServerError::Forbidden(err) => HttpResponse::Forbidden().json::<ErrorResponse>(err.into()),
            ServerError::UnprocessableEntity(json) => HttpResponse::BadRequest().json(json),
            ServerError::RequestTimeOut => HttpResponse::RequestTimeout().finish(),
            ServerError::MethodNotAllowed => HttpResponse::MethodNotAllowed().finish(),
            ServerError::BadGateway => HttpResponse::BadGateway().finish(),
            ServerError::NoContent => HttpResponse::NoContent().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
    // fn status_code(&self) -> StatusCode {
    //     match *self {
    //         Error::BadRequest(_) => StatusCode::BAD_REQUEST,
    //         Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    //         Error::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    //         Error::BadGateway => StatusCode::BAD_GATEWAY,
    //         Error::NotFound(_) => StatusCode::NOT_FOUND,
    //         Error::RequestTimeOut => StatusCode::REQUEST_TIMEOUT,
    //         Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
    //         Error::Conflict => StatusCode::CONFLICT,
    //         _ => StatusCode::OK,
    //     }
    // }
}

impl From<ValidationErrors> for ServerError {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();
        // transforms errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(|error| {
                    // dbg!(error) // <- Uncomment this if you want to see what error looks like
                    json!(error.message)
                })
                .collect();
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
pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}
