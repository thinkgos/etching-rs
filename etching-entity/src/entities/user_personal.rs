use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_personal")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: i64,
    pub real_name: String,
    pub id_card_type: u32,
    pub id_card: String,
    pub card_images: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub wechat: String,
    pub alipay: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}