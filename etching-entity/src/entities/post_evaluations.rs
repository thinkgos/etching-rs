use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "post_evaluations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub post_id: u64,
    pub user_id: u64,
    pub evaluation_user_id: Option<u64>,
    #[sea_orm(column_type = "Text")]
    pub evaluation: String,
    pub likes: u32,
    #[sea_orm(column_type = "Decimal(Some((10, 8)))", nullable)]
    pub latitude: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((11, 8)))", nullable)]
    pub longitude: Option<Decimal>,
    pub parent_evaluation_id: Option<u64>,
    pub top_evaluation_id: Option<u64>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: u64,
    pub status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::discovery_notices::Entity")]
    DiscoveryNotices,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentEvaluationId",
        to = "Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SelfRef2,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::TopEvaluationId",
        to = "Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    SelfRef1,
    #[sea_orm(has_many = "super::post_report::Entity")]
    PostReport,
    #[sea_orm(
        belongs_to = "super::posts::Entity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Posts,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::EvaluationUserId",
        to = "super::user::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    User1,
}

impl Related<super::discovery_notices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DiscoveryNotices.def()
    }
}

impl Related<super::post_report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostReport.def()
    }
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
