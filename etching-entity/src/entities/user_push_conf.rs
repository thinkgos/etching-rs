use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_push_conf")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: u64,
    pub orders: u8,
    pub system: u8,
    pub team: u8,
    pub hot: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
