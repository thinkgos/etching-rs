use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pub_pay_city")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub area_code: String,
    pub area_name: String,
    pub area_type: String,
    pub node_code: String,
    pub top_area_code1: String,
    pub top_area_code2: String,
    pub top_area_code3: String,
    pub ora_area_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
