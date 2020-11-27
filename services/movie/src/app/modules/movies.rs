use super::super::lib::*;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::FromRequest;
use futures::future::{err, ok, Ready};

pub async fn create_movies(req: web::Json<Movie>) -> HttpResponse {
    let movie: Movie = req.to_owned().into();
    match movie_db::insert(movie.to_owned()).await {
        Ok(_id) => HttpResponse::Created().json(Response::from(movie.to_owned())),
        Err(e) => ServerError::from(e).error_response(),
    }
}
