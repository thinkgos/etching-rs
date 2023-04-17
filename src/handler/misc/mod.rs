mod healthy;

pub use healthy::healthy;

use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("")
        .route("/v1/public/healthy", web::get().to(healthy))
        .route("/v1/public/hello", web::get().to(healthy))
    // .service(
    //     web::scope("/v2")
    //         .route("/public/healthy", web::get().to(healthy))
    //         .route("/public/hello", web::get().to(healthy)),
    // )
}

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     let scope = web::scope("")
//         .route("/v1/public/healthy", web::get().to(healthy))
//         .route("/v1/public/hello", web::get().to(healthy));
//     cfg.service(scope);
// }
