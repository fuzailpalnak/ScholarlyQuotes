//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quote_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub quote_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Categories,
    #[sea_orm(
        belongs_to = "super::quotes::Entity",
        from = "Column::QuoteId",
        to = "super::quotes::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Quotes,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::quotes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quotes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
