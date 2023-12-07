use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "pay_refund_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub orders_id: i64,
    pub pay_orders_id: i64,
    pub trade_no: String,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_tran_amt: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_tran_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_order_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_commission: Decimal,
    pub r#type: u32,
    pub status: u32,
    pub refund_reason: u32,
    pub refund_desc: String,
    pub refund_time: i64,
    pub interval: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
