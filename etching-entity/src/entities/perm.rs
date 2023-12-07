use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "perm")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub sub_id: i64,
    pub sub_type: u32,
    pub obj_id: i64,
    pub obj_type: u32,
    pub eft: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
