use actix_web::web;

use crate::handler::dict;
use crate::handler::passport;
use crate::handler::public;

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "etching server",
        description = include_str!("../../README.md"),
        contact (name = "thinkgos"),
    ),
    servers((url = "/api", description = "develop server")),
    paths(public::login, public::healthy))]
pub struct ApiDoc;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(passport::config)
            .configure(public::config)
            .configure(dict::config),
    );
}
