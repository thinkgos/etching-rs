use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_subjects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub company_name: String,
    pub juridical_person: String,
    pub id_card: String,
    pub uniform_code: String,
    pub address: String,
    pub capital: u32,
    pub creation_time: i64,
    pub scopes: String,
    pub license: String,
    pub photo: String,
    pub status: u32,
    pub deleted_at: i64,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}