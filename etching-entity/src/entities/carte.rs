use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "carte")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub nickname: String,
    pub mobile: String,
    pub email: String,
    pub wechat_no: String,
    pub tags: String,
    pub introduce: String,
    pub company: String,
    pub position: String,
    pub telephone: String,
    pub address: String,
    pub website: String,
    pub cover: String,
    pub is_default: bool,
    pub reason: String,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
    pub video: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}