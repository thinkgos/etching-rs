use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u64,
    pub status: String,
    pub r#type: String,
    pub luobo_id: String,
    pub passwd: String,
    pub name: String,
    pub nickname: String,
    pub mobile: String,
    pub email: Option<String>,
    pub avatar: String,
    pub sex: u32,
    pub register_ip: String,
    pub is_auth: String,
    pub is_crowdsourcing_agent: bool,
    pub pay_passwd: String,
    pub region: String,
    pub birthday: i64,
    pub last_login_at: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
    pub score: i64,
    pub count: i64,
    pub device_type: u32,
    pub is_old_passwd: bool,
    pub is_old_pay_passwd: bool,
    pub new_tran_net_member_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_followers::Entity")]
    PostFollowers,
    #[sea_orm(has_many = "super::posts::Entity")]
    Posts,
    #[sea_orm(has_many = "super::reset_trading_password::Entity")]
    ResetTradingPassword,
}

impl Related<super::post_followers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostFollowers.def()
    }
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl Related<super::reset_trading_password::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResetTradingPassword.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
