use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub demand_side_uid: i64,
    pub supply_side_uid: i64,
    pub crowd_uid: i64,
    pub r#type: u32,
    pub product_id: i64,
    pub product_name: String,
    pub location: String,
    #[sea_orm(column_type = "Decimal(Some((16, 7)))")]
    pub lat: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 7)))")]
    pub lon: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub estimated_amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((16, 2)))")]
    pub amount: Decimal,
    pub content: String,
    pub status: String,
    pub finish_at: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: i64,
    pub material_id: i64,
    pub supply_side_material_id: i64,
    pub label: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
