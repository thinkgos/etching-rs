//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "orders_pay_compensation_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub orders_id: i64,
    pub out_sub_acct_no: String,
    pub out_member_code: String,
    pub in_sub_acct_no: String,
    pub in_member_code: String,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub tran_amt: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub tran_fee: Decimal,
    pub tran_order_no: String,
    pub cnsmr_seq_no: String,
    pub front_seq_no: String,
    pub is_prepay: i8,
    pub pay_at: i64,
    pub status: u8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
