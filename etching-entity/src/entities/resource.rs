use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "resource")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub pid: i64,
    pub level: u32,
    pub tier: String,
    pub r#type: u32,
    pub code: String,
    pub permission: String,
    pub title: String,
    pub path: String,
    pub method: String,
    pub link_target: u32,
    pub icon: String,
    pub remark: String,
    pub sort: u32,
    pub visible: bool,
    pub is_enabled: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
