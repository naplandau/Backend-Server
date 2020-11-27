use actix_web::{guard, web, HttpResponse};

pub fn init_route(cfg: &mut web::ServiceConfig) {
    use super::lib::*;
    use crate::app::modules::*;
    cfg.service(
        web::scope("/api/v1").service(
            web::resource("movies")
                .route(web::post().to(create_movies))
                .default_service(web::route().to(|| HttpResponse::MethodNotAllowed())), // .route(web::put().to(|| )))
        ),
    );
}
