//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "member_property")]
pub enum MemberProperty {
    #[sea_orm(string_value = "00")]
    _00,
    #[sea_orm(string_value = "SH")]
    Sh,
}
