use actix_web::web;

use crate::handlers::{get_users, create_user}; // example handlers

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("", web::post().to(create_user)),
    );
}
