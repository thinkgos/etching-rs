use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "install")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub devices: String,
    pub region: String,
    pub platform: String,
    pub model: String,
    pub version: String,
    pub app_version: String,
    pub channel: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub additional: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}