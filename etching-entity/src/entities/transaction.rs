use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: i64,
    pub crowd_uid: i64,
    pub trade_no: String,
    pub voucher_id: u64,
    pub r#use: u32,
    pub channel: u32,
    pub r#type: u32,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub balance: Decimal,
    pub has_refund: bool,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub refund_amount: Decimal,
    pub receipt: String,
    pub desc: String,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}