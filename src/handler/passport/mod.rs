mod login;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/public/login", web::post().to(login::login))
        .route("/v1/logout", web::post().to(login::logout));
}
