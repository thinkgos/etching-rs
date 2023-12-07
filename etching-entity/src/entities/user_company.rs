use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_company")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: i64,
    pub name: String,
    pub raw_name: String,
    pub uniform_code: String,
    pub uniform_code_type: String,
    pub tax_level: String,
    pub mobile: String,
    pub email: String,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub registered_capital: Decimal,
    pub registered_capital_unit: u32,
    pub business_licences: String,
    pub tax_level_certificate: String,
    pub legal_id_card_front: String,
    pub legal_id_card_back: String,
    pub legal_id_card_type: u32,
    pub legal_id_card: String,
    pub legal_name: String,
    pub industry_id: i64,
    pub industry_name: String,
    pub staff_size: u32,
    pub founded_at: Option<DateTime>,
    pub stage: String,
    pub logo: String,
    pub website: String,
    pub introduction: String,
    pub apply_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
