use actix_web::{error, error::ResponseError, http::StatusCode, HttpRequest, HttpResponse};
use failure::Fail;
use serde_json::{Map as JsonMap, Value as JsonValue};
use validator::ValidationErrors;
#[derive(Debug, Fail, PartialEq)]
pub enum Error {
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
    InternalServerError(String),
    #[fail(display = "Method Not Allowed")]
    MethodNotAllowed,
    #[fail(display = "Bad Gateway")]
    BadGateway,
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
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            Error::NotFound(error) => HttpResponse::NotFound().json::<ErrorResponse>(error.into()),
            Error::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            Error::Forbidden(error) => {
                HttpResponse::Forbidden().json::<ErrorResponse>(error.into())
            }
            Error::RequestTimeOut => HttpResponse::RequestTimeout().finish(),
            Error::MethodNotAllowed => HttpResponse::MethodNotAllowed().finish(),
            Error::BadGateway => HttpResponse::BadGateway().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

impl From<ValidationErrors> for Error {
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

        Error::UnprocessableEntity(json!({
            "errors": err_map,
        }))
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
