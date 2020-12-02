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

pub async fn get_movies() -> HttpResponse {
    match movie_db::find_all().await {
        Ok(vec) => HttpResponse::Ok().json(ResponseList {
            data: vec_movie_to_vec_docs(vec),
            status: true,
            message: "Success".to_string()
        }),
        Err(_e) => {
            // error!("get_users: {:?}", _e);
            ServerError::InternalServerError.error_response()
        }
    }
}

pub async fn get_movie(id: web::Path<String>) -> HttpResponse {
    let res = movie_db::find_by_id(id.to_owned()).await.unwrap();
    match res {
        Some(movie) => HttpResponse::Ok().json(Response::from(movie.to_owned())),
        None => ServerError::NoContent.error_response()
    }
}

pub async fn delete_movie(id: web::Path<String>) -> HttpResponse {
    let data = movie_db::find_by_id(id.to_owned()).await.unwrap();
    match data {
        Some(_) => {
            let res = movie_db::delete_by_id(id.to_owned()).await;
            match res {
                Ok(op) => match op {
                    Some(u) => HttpResponse::Ok().json(Response::from(u.to_owned())),
                    None => ServerError::NoContent.error_response(),
                },
                Err(_) => ServerError::InternalServerError.error_response(),
            }
        }
        None => ServerError::NoContent.error_response()
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
            description: movie.description.to_owned(),
            format: movie.format.to_owned(),
            suitability: movie.suitability.to_owned(),
            duration: movie.duration.to_owned(),
            directors: movie.directors.to_owned(),
            language: movie.language.to_owned(),
            status: movie.status.to_owned(),
            nation: movie.nation.to_owned(),
            release: movie.release.to_owned(),
            rating: None,
            keyword: movie.keyword.to_owned().to_vec(),
            trailer: movie.trailer.to_owned().to_vec(),
            category: movie.trailer.to_owned().to_vec(),
            ibm: None,
            ticket_sold: None,
            studio: movie.studio.to_owned(),
            created_by: "".to_owned(),
            created_time_dt: current_time.timestamp(),
            updated_by: "".to_owned(),
            updated_time_dt: current_time.timestamp()
        }
    }
}

fn vec_movie_to_vec_docs(vec: Vec<Movie>) -> Vec<Document> {
    let mut res: Vec<Document> = Vec::new();
    for movie in vec.iter() {
        res.push(get_sub_field(&bson::to_document(&movie).unwrap()));
    }
    res
}