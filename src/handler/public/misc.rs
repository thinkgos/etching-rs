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
    context_path = "/v1",
    responses(
        (status = StatusCode::OK, body = inline(HealthyResponse))
    ),
)]
#[get("/public/healthy")]
pub async fn healthy() -> impl Responder {
    HttpResponse::Ok().json(HealthyResponse {
        status: "running".to_string(),
    })
}
