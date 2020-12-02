use actix_web::{guard, web, HttpResponse};

use super::{lib::get_mongo, modules::get_movies};


pub fn init_route(cfg: &mut web::ServiceConfig) {
    use super::lib::*;
    use crate::app::modules::*;
    cfg.service(
        web::scope("/api/v1")
            .guard(guard::Header("content-type", "application/json"))
            .service(
                web::resource("movies")
                    .route(web::post().to(create_movies))
                    .route(web::get().to(get_movies))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())), // .route(web::put().to(|| )))
            )
            .service(
                web::resource("movies/{id}")
                    .route(web::delete().to(delete_movie))
                    .route(web::get().to(get_movie))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())), // .route(web::put().to(|| )))
            ),
    );
}
