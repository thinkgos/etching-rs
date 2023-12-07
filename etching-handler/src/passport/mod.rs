mod logout;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/logout", web::post().to(logout::logout));
}
