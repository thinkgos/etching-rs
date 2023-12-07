use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pay_info")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub orders_id: i64,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub supply_side_taxes: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub supply_side_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub demand_side_taxes: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub demand_side_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub commission: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub total_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_commission: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_total_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub plat_commission_ratio: Decimal,
    pub status: String,
    pub pay_time: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
    pub channel: u32,
    pub polling_at: i64,
    pub started_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
