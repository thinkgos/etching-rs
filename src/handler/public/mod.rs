mod misc;
mod passport;

use actix_web::web;
pub use misc::*;
pub use passport::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(passport::login);
    cfg.service(misc::healthy);
}
