use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "feedback")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub client_type: String,
    pub mobile: String,
    pub opinion: String,
    pub image: String,
    pub remark: String,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
