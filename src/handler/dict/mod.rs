use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};
// use sea_query::{Expr, Query};
use entity::{dict, dict::Column as DictColumn, prelude::Dict};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, IntoParams)]
struct ListDictRequest {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ListDictResponse {
    total: i64,
    page: i64,
    per_page: i64,
    #[schema(inline)]
    list: Vec<dict::Model>,
}
/// 获取字典列表
#[utoipa::path(
    tag = "字典",
    context_path = "/v1",
    params(ListDictRequest),
    responses(
        (status = StatusCode::OK, body = inline(ListDictResponse))
    ),
)]
#[get("/dict")]
pub async fn list_dict(db_conn: web::Data<DatabaseConnection>) -> impl Responder {
    let rows: Vec<dict::Model> = Dict::find().limit(100).all(db_conn.as_ref()).await.unwrap();

    Dict::find()
        .select_only()
        .columns(vec![
            DictColumn::Id,
            DictColumn::Key,
            DictColumn::Name,
            DictColumn::IsPin,
            DictColumn::Remark,
            DictColumn::CreatedAt,
            DictColumn::UpdatedAt,
        ])
        .all(db_conn.as_ref())
        .await
        .unwrap();
    let total = rows.len() as i64;
    HttpResponse::Ok().json(ListDictResponse {
        total,
        page: 1,
        per_page: 20,
        list: rows,
    })
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub(crate) struct GetDictRequest {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetDictResponse {
    #[schema(inline)]
    pub entity: Option<dict::Model>,
}

/// 获取字典详情
#[utoipa::path(
    tag = "字典",
    context_path = "/v1",
    params(GetDictRequest),
    responses(
        (status = StatusCode::OK, body = inline(GetDictResponse))
    ),
)]
#[get("/dict/{id}")]
pub async fn get_dict(
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
