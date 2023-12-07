use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "self_employed")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: u64,
    pub company_name: String,
    pub juridical_person: String,
    pub id_card: String,
    pub uniform_code: String,
    pub address: String,
    pub capital: i32,
    pub creation_time: i64,
    pub scopes: String,
    pub license: String,
    pub photo: String,
    pub reject: String,
    pub status: u8,
    pub deleted_at: i64,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
