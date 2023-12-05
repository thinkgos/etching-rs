use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "invoice_commission")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub uid: i64,
    pub agent_commission_settle_type: u32,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub agent_commission_ratio: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub agent_commission: Decimal,
    pub agent_id: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
