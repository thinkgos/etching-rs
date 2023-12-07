use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "topic")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub category_id: u64,
    pub name: String,
    pub status: String,
    pub cover: String,
    pub detail: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub reads: i32,
    pub deleted_at: i64,
    pub participation: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::posts::Entity")]
    Posts,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
