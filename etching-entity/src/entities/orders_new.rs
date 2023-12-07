use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "orders_new")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub demand_side_uid: i64,
    pub supply_side_uid: i64,
    pub product_id: i64,
    pub product_name: String,
    pub label: String,
    pub cycle: i64,
    pub content: String,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub lon: Decimal,
    pub location: String,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub plat_commission_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub total_amount: Decimal,
    pub status: u32,
    pub send_at: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}