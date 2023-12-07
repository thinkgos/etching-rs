use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "invoice_pending")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub orders_id: i64,
    pub uid: i64,
    pub crowd_uid: i64,
    pub has_apply: i8,
    pub has_abandon: i8,
    pub has_selected: i8,
    pub created_at: DateTime,
    pub associate_bind_id: i64,
    pub invoice_body_id: i64,
    pub invoice_body_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}