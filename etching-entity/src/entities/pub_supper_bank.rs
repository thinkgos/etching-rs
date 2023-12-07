use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pub_supper_bank")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub no: String,
    pub name: String,
    pub agent_code: String,
    pub apr_code: String,
    pub status: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}