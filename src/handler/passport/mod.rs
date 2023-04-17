mod login;
mod logout;

pub use login::login;
pub use logout::logout;

use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("")
        .route("/v1/public/login", web::get().to(login))
        .route("/v1/logout", web::get().to(logout))
}
