use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub passwd: String,
    pub real_name: String,
    pub nickname: String,
    pub mobile: String,
    pub avatar: String,
    pub email: String,
    pub qq: String,
    pub wx: String,
    pub gender: u32,
    pub last_login_ip: String,
    pub last_login_loc: String,
    pub last_login_time: i64,
    pub is_admin: bool,
    pub need_change: bool,
    pub status: u32,
    pub creator_id: i64,
    pub create_by: String,
    pub update_by: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
