mod healthy;
mod login;

use actix_web::web::{self};

pub fn config_v1(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::scope("/public")
    //         .route("/healthy", web::get().to(healthy::healthy))
    //         .route("/login", web::post().to(login::login)),
    // );
    cfg.service(login::login);
    cfg.service(healthy::healthy);
}

pub fn config_v2(cfg: &mut web::ServiceConfig) {
    cfg.service(healthy::healthy);
}
