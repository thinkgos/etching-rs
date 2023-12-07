use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "stores_qualified")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub stores_id: u64,
    pub name: String,
    pub annexes: String,
    pub reject: String,
    pub status: i8,
    pub product_id: u64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}