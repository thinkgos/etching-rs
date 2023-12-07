use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "daily_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub orders_id: u64,
    pub user_id: u64,
    pub annexes: String,
    pub content: Option<String>,
    pub working_hours: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}