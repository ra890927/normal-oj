use super::_entities::courses;
pub use super::_entities::courses::{ActiveModel, Entity, Model};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::entity::prelude::*;

impl super::_entities::courses::Model {
    /// finds a course by the provided name
    ///
    /// # Errors
    ///
    /// When could not find user by the given name or DB query error
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> ModelResult<Self> {
        Self::find_by_column(db, courses::Column::Name, name).await
    }

    /// finds a course by id
    ///
    /// # Errors
    ///
    /// When could not find user by id or DB query error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        Self::find_by_column(db, courses::Column::Id, id).await
    }

    async fn find_by_column(
        db: &DatabaseConnection,
        column: impl sea_orm::ColumnTrait,
        value: impl Into<sea_orm::Value> + Send,
    ) -> ModelResult<Self> {
        let course = courses::Entity::find()
            .filter(column.eq(value))
            .one(db)
            .await?;
        course.ok_or_else(|| ModelError::EntityNotFound)
    }
}

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
