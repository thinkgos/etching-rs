//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pab_transfer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub front_seq_no: String,
    pub sub_acct_no: String,
    pub tran_net_member_code: String,
    pub in_acct_type: String,
    pub in_acct_no: String,
    pub in_acct_name: String,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub tran_amt: Decimal,
    pub ccy: String,
    pub bank_name: String,
    pub accounting_date: String,
    pub remark: String,
    pub accounting_date_unix: i64,
    pub status: u32,
    pub operator: String,
    pub operating_at: i64,
    pub compensation_sub_acct_no: String,
    pub compensation_uid: i64,
    pub compensation_username: String,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub compensation_tran_amt: Decimal,
    pub register_bill_id: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}