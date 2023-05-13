//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pab_bind_card_apply")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub card_no: String,
    pub bank_name: String,
    pub bank_alias_name: String,
    pub bank_branch_name: String,
    pub branch_bank_no: String,
    pub super_bank_no: String,
    pub bank_agent_code: String,
    pub mobile: String,
    pub status: u8,
    pub do_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
