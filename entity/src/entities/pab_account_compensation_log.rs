//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use super::sea_orm_active_enums::MemberProperty;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pab_account_compensation_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub relate_id: i64,
    pub uid: i64,
    pub sub_acct_no: String,
    pub member_property: MemberProperty,
    pub mobile: String,
    pub shop_name: String,
    pub member_name: String,
    pub member_global_type: u8,
    pub member_global_id: String,
    pub card_images: String,
    pub status: u8,
    pub reason: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
