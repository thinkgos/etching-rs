use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "stats_user_retention")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub stats_date: i64,
    pub dnu: i64,
    pub dau: i64,
    pub rolling_wau: i64,
    pub rolling_mau: i64,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no1: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no2: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no3: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no4: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no5: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no6: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no7: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no14: Decimal,
    #[sea_orm(column_type = "Decimal(Some((12, 4)))")]
    pub day_no30: Decimal,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
