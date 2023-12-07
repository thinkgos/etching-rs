use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "pub_pay_bank")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub branch_no: String,
    pub name: String,
    pub agent_code: String,
    pub apr_code: String,
    pub node_code: String,
    pub city_code: String,
    pub status: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
