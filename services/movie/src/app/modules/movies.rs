use super::super::lib::*;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::FromRequest;
use futures::future::{err, ok, Ready};

pub async fn create_movies(req: web::Json<AddMovie>) -> HttpResponse {
    let movie: Movie = req.to_owned().into();
    match movie_db::insert(movie.to_owned()).await {
        Ok(_id) => HttpResponse::Created().json(Response::from(movie.to_owned())),
        Err(e) => ServerError::from(e).error_response(),
    }
}

impl From<Movie> for Response {
    fn from(movie: Movie) -> Self {
        Response {
            data: get_sub_field(&bson::to_document(&movie).unwrap()),
            message: "success".to_string(),
            status: true,
        }
    }
}

impl From<AddMovie> for Movie {
    fn from(movie: AddMovie) -> Self {
        let current_time = Utc::now();
        Movie {
            id: String::from("movie_") + &Uuid::new_v4().to_simple().to_string(),
            tittle: movie.tittle.to_owned(),
            description: "".to_owned(),
            format: "".to_owned(),
            suitability: [].to_owned().to_vec(),
            duration: bson::DateTime(current_time),
            directors: [].to_owned().to_vec(),
            language: "".to_owned(),
            status: 0,
            nation: "".to_owned(),
            release: bson::DateTime(current_time),
            rating: 0,
            keyword: [].to_owned().to_vec(),
            trailer: [].to_owned().to_vec(),
            category: [].to_owned().to_vec(),
            ibm: 0,
            ticket_sold: 0,
            studio: "".to_owned(),
            created_by: "".to_owned(),
            created_time_dt: bson::DateTime(current_time),
            updated_by: "".to_owned(),
            updated_time_dt: bson::DateTime(current_time),
        }
    }
}
