use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user_id: u64,
    pub topic_id: u64,
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub audio_url: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
    pub media_types: String,
    #[sea_orm(column_type = "Decimal(Some((10, 8)))", nullable)]
    pub latitude: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((11, 8)))", nullable)]
    pub longitude: Option<Decimal>,
    pub likes: i32,
    pub replies: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<i64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub additional: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_evaluations::Entity")]
    PostEvaluations,
    #[sea_orm(has_many = "super::post_followers::Entity")]
    PostFollowers,
    #[sea_orm(has_many = "super::post_report::Entity")]
    PostReport,
    #[sea_orm(
        belongs_to = "super::topic::Entity",
        from = "Column::TopicId",
        to = "super::topic::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Topic,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    User,
}

impl Related<super::post_evaluations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostEvaluations.def()
    }
}

impl Related<super::post_followers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostFollowers.def()
    }
}

impl Related<super::post_report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostReport.def()
    }
}

impl Related<super::topic::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Topic.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
