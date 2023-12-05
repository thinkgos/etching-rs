use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u64,
    pub name: String,
    pub status: String,
    pub category_id: String,
    pub cover: String,
    pub image: String,
    pub detail: String,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub price: Decimal,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
    pub synonym: String,
    pub is_severity: bool,
    pub sequence: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
