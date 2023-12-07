use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "virtual_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub mobile: String,
    pub city: String,
    pub address: String,
    pub product_id: i64,
    pub remark: String,
    #[sea_orm(column_type = "Decimal(Some((16, 7)))")]
    pub lat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 7)))")]
    pub lng: Decimal,
    pub precise: bool,
    pub confidence: u32,
    pub comprehension: u32,
    pub status: u32,
    pub reg_at: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
