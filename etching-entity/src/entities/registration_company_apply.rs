use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "registration_company_apply")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub user_type: u32,
    pub is_crowdsourcing_agent: bool,
    pub name: String,
    pub raw_name: String,
    pub uniform_code: String,
    pub tax_level: String,
    pub mobile: String,
    pub email: String,
    pub business_licence: String,
    pub tax_level_certificate: String,
    pub legal_id_card_front: String,
    pub legal_id_card_back: String,
    pub legal_id_card_type: u32,
    pub legal_id_card: String,
    pub legal_name: String,
    pub apply_name: String,
    pub audit_status: u32,
    pub audit_id: i64,
    pub audit_name: String,
    pub audit_at: i64,
    pub reason: String,
    pub code: String,
    pub is_success: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
