use super::sea_orm_active_enums::MemberProperty;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "pab_register_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub sub_acct_no: String,
    pub member_property: MemberProperty,
    pub r#type: u32,
    pub order_no: String,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amt: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub fee: Decimal,
    pub remark: String,
    pub cnsmr_seq_no: String,
    pub front_seq_no: String,
    pub revoke_cnsmr_seq_no: String,
    pub revoke_front_seq_no: String,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
