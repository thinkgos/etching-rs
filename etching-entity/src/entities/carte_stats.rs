use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "carte_stats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub carte_id: i64,
    pub view: i64,
    pub star: i64,
    pub favorite: i64,
    pub call: i64,
    pub shared: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
