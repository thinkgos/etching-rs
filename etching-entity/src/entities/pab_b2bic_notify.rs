use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pab_b2bic_notify")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub host_date: String,
    pub acct_date: String,
    pub tx_time: String,
    pub host_trace: String,
    pub buss_seq_no: String,
    pub detail_serial_no: i64,
    pub out_node: String,
    pub out_bank_no: String,
    pub out_bank_name: String,
    pub out_acct_no: String,
    pub out_acct_name: String,
    pub in_node: String,
    pub in_bank_no: String,
    pub in_bank_name: String,
    pub in_acct_no: String,
    pub in_acct_name: String,
    pub tran_amount: String,
    pub tran_fee: String,
    pub acct_balance: String,
    pub purpose: String,
    pub abstract_str: String,
    pub abstract_str_desc: String,
    pub sn: String,
    pub tran_date: String,
    pub tran_time: String,
    pub tran_code: String,
    pub return_code: String,
    pub return_msg: String,
    pub raw_data: String,
    pub transfer_stt: String,
    pub transfer_voucher: String,
    pub uid: i64,
    pub signed_subject_uid: i64,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub actual_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub fee: Decimal,
    pub remark: String,
    pub status: u32,
    pub retry: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}