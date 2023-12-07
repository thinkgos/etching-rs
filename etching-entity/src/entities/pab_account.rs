use super::sea_orm_active_enums::MemberProperty;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "pab_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub sub_acct_no: String,
    pub member_property: MemberProperty,
    pub tran_net_member_code: String,
    pub mobile: String,
    pub shop_id: String,
    pub shop_name: String,
    pub has_behavior_record: bool,
    pub apply_new_mobile: String,
    pub bk_mer_sub_acct_no: String,
    pub bk_mer_tran_net_member_code: String,
    pub bk_normal_sub_acct_no: String,
    pub bk_normal_tran_net_member_code: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
