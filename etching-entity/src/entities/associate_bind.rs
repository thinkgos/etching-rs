use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "associate_bind")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub task_no: String,
    pub task_count: u32,
    pub invoice_id: i64,
    pub is_online: bool,
    pub invoice_body_id: i64,
    pub invoice_body_name: String,
    pub operator_id: i64,
    pub operator: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
