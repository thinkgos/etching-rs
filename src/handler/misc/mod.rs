mod healthy;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/public/healthy", web::get().to(healthy::healthy));
}
