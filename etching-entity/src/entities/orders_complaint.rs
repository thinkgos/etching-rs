use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "orders_complaint")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub orders_id: i64,
    pub user_id: i64,
    pub reason: String,
    pub annexes: String,
    pub content: String,
    pub orders_previous_status: String,
    pub result: u32,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
