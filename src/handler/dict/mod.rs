use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};
use serde::{Deserialize, Serialize};

use entity::{dict, dict::Entity as Dict};

pub fn config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dict")
            .route("", web::get().to(list_dict))
            .route("/{id}", web::get().to(get_dict)),
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct ListDictResponse {
    pub list: Vec<dict::Model>,
}

pub(crate) async fn list_dict(db_conn: web::Data<DatabaseConnection>) -> impl Responder {
    let rows: Vec<dict::Model> = Dict::find().limit(100).all(db_conn.as_ref()).await.unwrap();
    HttpResponse::Ok().json(ListDictResponse { list: rows })
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GetDictRequest {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetDictResponse {
    pub entity: Option<dict::Model>,
}

pub(crate) async fn get_dict(
    db_conn: web::Data<DatabaseConnection>,
    path: web::Path<GetDictRequest>,
) -> impl Responder {
    let row: Option<dict::Model> = Dict::find_by_id(path.id)
        .one(db_conn.as_ref())
        .await
        .unwrap();
    tracing::info!("{:?}", row);

    HttpResponse::Ok().json(GetDictResponse { entity: row })
}
