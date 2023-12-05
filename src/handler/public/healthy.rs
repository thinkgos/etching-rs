use actix_web::get;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HealthyResponse {
    status: String,
}
#[get("/public/healthy")]
pub(crate) async fn healthy() -> impl Responder {
    HttpResponse::Ok().json(HealthyResponse {
        status: "running".to_string(),
    })
}
