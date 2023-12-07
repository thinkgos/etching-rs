use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pab_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub uid: i64,
    pub inter_id: String,
    pub service_id: String,
    pub provider_id: i64,
    pub txn_return_code: String,
    pub txn_return_msg: String,
    pub cnsmr_seq_no: String,
    pub front_seq_no: String,
    pub errmsg: String,
    #[sea_orm(column_type = "Text")]
    pub request_data: String,
    #[sea_orm(column_type = "Text")]
    pub response_data: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
