use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "banner")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub kind_id: i64,
    pub kind: u32,
    pub name: String,
    pub image: String,
    pub desc: String,
    pub sort: u32,
    pub visible: i8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
