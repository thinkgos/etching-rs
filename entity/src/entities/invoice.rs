//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "invoice")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub invoice_no: String,
    pub uid: i64,
    pub task_no: String,
    pub task_count: u16,
    pub uniform_code: String,
    pub title_type: u8,
    pub title: String,
    pub company_address: String,
    pub company_telephone: String,
    pub bank_name: String,
    pub bank_card_no: String,
    pub receiver_name: String,
    pub receiver_mobile: String,
    pub receiver_address: String,
    pub receiver_email: String,
    pub remark: String,
    pub invoice_type: u8,
    pub invoice_tax_ratio_id: i64,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub tax_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub taxes_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub allowance_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub order_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub plat_commission: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub shared_economy_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub invoice_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub taxes: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub excluding_tax_invoice_amount: Decimal,
    pub invoice_body_id: i64,
    pub invoice_body_name: String,
    pub is_auto_add_invoice_body: i8,
    pub invoice_body_message: String,
    pub reason: String,
    pub apply_status: u8,
    pub status: u8,
    pub operator_id: i64,
    pub operator_name: String,
    pub invoice_at: i64,
    pub apply_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
