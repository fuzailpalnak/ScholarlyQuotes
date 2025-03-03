//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quote_of_the_day")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub quote_id: i32,
    pub language: String,
    pub date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::quotes::Entity",
        from = "Column::QuoteId",
        to = "super::quotes::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Quotes,
}

impl Related<super::quotes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quotes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
