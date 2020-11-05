use super::super::lib::*;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok".into(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
