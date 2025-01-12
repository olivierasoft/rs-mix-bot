//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_team::Entity")]
    UserTeam,
}

impl Related<super::user_team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTeam.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_team::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_team::Relation::Team.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
