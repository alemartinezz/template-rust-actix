use actix_web::web;
use crate::handlers::user::{get_user, create_user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_user))
            .route(web::post().to(create_user)),
    );
}
