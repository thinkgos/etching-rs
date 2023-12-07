use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "sys_district")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pid: i32,
    pub district_code: String,
    pub name: String,
    pub parent_district_code: String,
    pub short_name: String,
    pub district_type: u32,
    pub city_code: String,
    pub zipcode: String,
    pub merger_name: String,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub longitude: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 7)))")]
    pub latitude: Decimal,
    pub pinyin: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
