use super::errors::Error;
use actix_web::{body::Body, web::{HttpResponse, Json}};
use serde::Serialize;

pub fn respond_json<T>(data: T) -> Result<Json<T>,Error>
where
    T: Serialize,
{
    Ok(Json(data))
}