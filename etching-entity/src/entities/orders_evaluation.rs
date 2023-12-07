use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "orders_evaluation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub orders_id: u64,
    pub user_id: u64,
    pub evaluators: u64,
    pub score: u8,
    pub tags: String,
    pub content: Option<String>,
    pub annexes: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
    pub status: String,
    pub anonymity: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
