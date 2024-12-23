//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pages")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub url: String,
    pub body: String,
    pub website_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::websites::Entity",
        from = "Column::WebsiteId",
        to = "super::websites::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Websites,
}

impl Related<super::websites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Websites.def()
    }
}
