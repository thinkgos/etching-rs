use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "carte_invoice")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub carte_id: i64,
    pub company: String,
    pub uniform_code: String,
    pub address: String,
    pub telephone: String,
    pub bank_name: String,
    pub bank_card_no: String,
    pub visible: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
