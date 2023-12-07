use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "team_auto_invite")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub real_name: String,
    pub id_card: String,
    pub mobile: String,
    pub has_join: bool,
    pub member_uid: i64,
    pub remark: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
