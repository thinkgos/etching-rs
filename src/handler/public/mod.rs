mod healthy;
mod login;

use actix_web::web;

pub fn config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/public")
            .route("/healthy", web::get().to(healthy::healthy))
            .route("/login", web::post().to(login::login)),
    );
}
