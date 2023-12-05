use actix_web::get;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
struct HealthyResponse {
    // running status
    status: String,
}

/// 健康检查
#[utoipa::path(
    tag = "开放接口",
    responses(
        (status = 200, body = inline(HealthyResponse))
    ),
)]
#[get("/v1/public/healthy")]
pub async fn healthy() -> impl Responder {
    HttpResponse::Ok().json(HealthyResponse {
        status: "running".to_string(),
    })
}
