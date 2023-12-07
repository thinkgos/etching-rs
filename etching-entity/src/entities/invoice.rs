use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "invoice")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub invoice_no: String,
    pub uid: i64,
    pub invoice_apply_id: i64,
    pub task_no: String,
    pub task_count: u32,
    pub uniform_code: String,
    pub title_type: u32,
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
    pub invoice_type: u32,
    pub invoice_tax_ratio_id: i64,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub tax_ratio: Decimal,
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
    pub is_auto_add_invoice_body: bool,
    pub invoice_body_message: Json,
    pub reason: String,
    pub status: u32,
    pub operator_id: i64,
    pub operator_name: String,
    pub invoice_at: i64,
    pub apply_at: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
