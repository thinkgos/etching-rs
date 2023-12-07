use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "discovery_notices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub sender: Option<u64>,
    pub receiver: u64,
    pub send_at: i64,
    pub read_at: i64,
    pub evaluation_id: Option<u64>,
    pub post_id: Option<u64>,
    pub r#type: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post_evaluations::Entity",
        from = "Column::EvaluationId",
        to = "super::post_evaluations::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PostEvaluations,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Receiver",
        to = "super::user::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Sender",
        to = "super::user::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    User1,
}

impl Related<super::post_evaluations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostEvaluations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}