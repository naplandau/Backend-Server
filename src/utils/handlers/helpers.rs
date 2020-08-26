use super::errors::Error;
use actix_web::{body::Body, web::{HttpResponse, Json}};
use serde::Serialize;

pub fn respond_json<T>(data: T) -> Result<Json<T>,Error>
where
    T: Serialize,
{
    Ok(Json(data))
}
#[allow(dead_code)]
pub fn respond_ok() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}