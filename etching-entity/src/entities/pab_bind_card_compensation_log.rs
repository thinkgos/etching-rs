use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "pab_bind_card_compensation_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub relate_id: i64,
    pub card_no: String,
    pub is_success: bool,
    pub status: u32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
