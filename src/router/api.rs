use actix_web::web;

use crate::handler::dict;
use crate::handler::passport;
use crate::handler::public;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1")
                .configure(passport::config_v1)
                .configure(public::config_v1)
                .configure(dict::config_v1),
        ),
    );
}
