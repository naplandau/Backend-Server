use super::super::lib::*;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn get_health() -> Result<Json<HealthResponse>, Error> {
    respond_json(HealthResponse {
        status: "Ok".into(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
// pub fn get_super_health() -> impl Future<Output = Result<Item, Error>>{
//     Item("Hello")
// }
