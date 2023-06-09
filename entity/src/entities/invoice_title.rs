//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "invoice_title")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub invoice_type: u32,
    pub r#type: u32,
    pub title: String,
    pub uniform_code: String,
    pub company_address: String,
    pub company_telephone: String,
    pub bank_name: String,
    pub bank_card_no: String,
    pub receiver_name: String,
    pub receiver_email: String,
    pub receiver_mobile: String,
    pub receiver_address: String,
    pub is_default: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
