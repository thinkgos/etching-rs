use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "geo_map")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub pid: String,
    pub deep: String,
    pub name: String,
    pub ext_path: String,
    #[sea_orm(column_type = "custom(\"GEOMETRY\")")]
    pub geo: String,
    #[sea_orm(column_type = "custom(\"GEOMETRY\")")]
    pub polygon: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
