use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};
use serde::{Deserialize, Serialize};

use entity::{dict, dict::Entity as Dict};

#[derive(Debug, Serialize, Deserialize)]
struct HealthyResponse {
    status: String,
}

pub(crate) async fn healthy(db_conn: web::Data<DatabaseConnection>) -> impl Responder {
    let cc: Vec<dict::Model> = Dict::find().limit(100).all(db_conn.as_ref()).await.unwrap();
    tracing::info!("{:?}", cc);

    HttpResponse::Ok().json(HealthyResponse {
        status: "running".to_string(),
    })
}
