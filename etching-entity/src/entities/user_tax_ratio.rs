use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_tax_ratio")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub tpl_tax_ratio_id: i64,
    pub invoice_type: u32,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub tax_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub taxes_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub allowance_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub agent_commission_ratio: Decimal,
    pub operator_id: i64,
    pub operator: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}