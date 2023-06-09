//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "news")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub sys_user_id: i64,
    pub title: Option<String>,
    pub sub_title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub index: i64,
    pub cover: Option<String>,
    pub is_enable: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_user::Entity",
        from = "Column::SysUserId",
        to = "super::sys_user::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SysUser,
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
