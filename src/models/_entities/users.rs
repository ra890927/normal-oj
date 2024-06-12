//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::Role;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pid: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    #[sea_orm(unique)]
    pub api_key: String,
    #[sea_orm(unique)]
    pub name: String,
    pub reset_token: Option<String>,
    pub reset_sent_at: Option<DateTime>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTime>,
    pub email_verified_at: Option<DateTime>,
    pub role: Role,
    pub displayed_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::courses::Entity")]
    Courses,
    #[sea_orm(has_many = "super::problems::Entity")]
    Problems,
    #[sea_orm(has_many = "super::submissions::Entity")]
    Submissions,
}

impl Related<super::courses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Courses.def()
    }
}

impl Related<super::problems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problems.def()
    }
}

impl Related<super::submissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submissions.def()
    }
}
