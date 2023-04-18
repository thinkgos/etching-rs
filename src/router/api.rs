use actix_web::web;

use crate::handler::misc;
use crate::handler::passport;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(passport::config)
            .configure(misc::config),
    );
}
