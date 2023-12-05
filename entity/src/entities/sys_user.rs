use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    pub sex: u32,
    pub last_login_ip: String,
    pub last_login_loc: String,
    pub last_login_time: Option<DateTime>,
    pub is_admin: bool,
    pub need_change: bool,
    pub status: u32,
    pub creator: String,
    pub updator: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::news::Entity")]
    News,
}

impl Related<super::news::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::News.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
