//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_position")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: u64,
    #[sea_orm(column_type = "Decimal(Some((10, 8)))", nullable)]
    pub latitude: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((11, 8)))", nullable)]
    pub longitude: Option<Decimal>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
