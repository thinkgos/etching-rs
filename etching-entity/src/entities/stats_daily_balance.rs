use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "stats_daily_balance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub stats_date: i64,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub wechat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub alipay: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub bank: Decimal,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}