use actix_web::get;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, QuerySelect};
use serde::{Deserialize, Serialize};

use entity::{dict, dict::Entity as Dict};

use crate::runtime::Runtime;

#[derive(Debug, Serialize, Deserialize)]
struct HealthyResponse {
    status: String,
}
#[get("/public/healthy")]
pub(crate) async fn healthy(rt: web::Data<Runtime>) -> impl Responder {
    let cc: Vec<dict::Model> = Dict::find().limit(100).all(&rt.db_pool).await.unwrap();
    tracing::info!("{:?}", cc);

    HttpResponse::Ok().json(HealthyResponse {
        status: "running".to_string(),
    })
}
