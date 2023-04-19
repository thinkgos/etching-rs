mod logout;

use actix_web::web;

pub fn config_v1(cfg: &mut web::ServiceConfig) {
    cfg.route("/logout", web::post().to(logout::logout));
}
