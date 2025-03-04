//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "queue")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub length: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::guild::Entity",
        from = "Column::Id",
        to = "super::guild::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Guild,
    #[sea_orm(has_many = "super::queue_user::Entity")]
    QueueUser,
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}

impl Related<super::queue_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QueueUser.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::queue_user::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::queue_user::Relation::Queue.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
