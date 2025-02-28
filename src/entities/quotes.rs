//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quotes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub author: String,
    #[sea_orm(column_type = "Text")]
    pub quote: String,
    #[sea_orm(column_type = "Text", column_name = "reference", nullable)]
    pub reference: Option<String>,
    pub language: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::quote_category::Entity")]
    QuoteCategory,
}

impl Related<super::quote_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuoteCategory.def()
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        super::quote_category::Relation::Categories.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::quote_category::Relation::Quotes.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
