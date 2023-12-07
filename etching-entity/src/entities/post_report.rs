use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "post_report")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: u64,
    pub post_id: u64,
    pub reply_id: Option<u64>,
    pub desc: String,
    pub created_at: DateTime,
    pub deleted_at: u64,
    pub r#type: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post_evaluations::Entity",
        from = "Column::ReplyId",
        to = "super::post_evaluations::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PostEvaluations,
    #[sea_orm(
        belongs_to = "super::posts::Entity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Posts,
}

impl Related<super::post_evaluations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostEvaluations.def()
    }
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
