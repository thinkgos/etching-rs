use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "app_version")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub client_type: u32,
    pub is_force: bool,
    pub version: String,
    pub name: String,
    pub app_url: String,
    pub content: String,
    pub remark: String,
    pub hint: String,
    pub button_label: String,
    pub is_enabled: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
