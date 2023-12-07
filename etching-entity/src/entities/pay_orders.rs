use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "pay_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub pay_id: i64,
    pub orders_id: i64,
    pub prepay_channel_id: String,
    pub channel_id: String,
    pub channel: i32,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub order_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub commission: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub demand_cost: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub supply_cost: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub confirm_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_order_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_commission: Decimal,
    pub refund_type: u32,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
