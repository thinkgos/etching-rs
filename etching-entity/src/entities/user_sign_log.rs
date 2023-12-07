use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "user_sign_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub r#type: u32,
    pub signed_subject_uid: i64,
    pub signed_subject_card_no: String,
    pub signed_subject_company_name: String,
    #[sea_orm(column_type = "Decimal(Some((12, 6)))")]
    pub crowdsourcing_ratio: Decimal,
    pub remark: String,
    pub operator_id: i64,
    pub operator: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
