pub mod descriptions;
pub mod test_case;

use super::_entities::{self, prelude::Problems, problems};
use crate::models::transform_db_error;

pub use _entities::problems::{ActiveModel, Model};
use loco_rs::model::{ModelError, ModelResult};
use num_derive::FromPrimitive;
use sea_orm::{entity::prelude::*, ActiveValue, Order, QueryOrder, TransactionTrait};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("user is not permitted to this operation")]
    PermissionDenied,
}

#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, FromPrimitive)]
#[repr(i8)]
pub enum Visibility {
    Show = 0,
    Hidden = 1,
}

#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, FromPrimitive)]
#[repr(i8)]
pub enum Type {
    Normal = 0,
    FillInTemplate = 1,
    Handwritten = 2,
}

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

#[derive(Debug, Deserialize)]
pub struct AddParams {
    pub owner: _entities::users::Model,
    /// list of course names
    pub courses: Vec<String>,
    pub name: String,
    /// Problem status, control its visibility
    pub status: Option<Visibility>,
    /// Problem description struct
    pub description: descriptions::AddParams,
    pub r#type: Option<Type>,
    pub allowed_language: Option<i32>,
    pub quota: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub viewer: _entities::users::Model,
    pub offset: Option<usize>,
    pub count: Option<usize>,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub course: Option<String>,
}

impl _entities::problems::Model {
    /// Create a problem without test case binary
    ///
    /// # Errors
    ///
    /// - When could not save the problem into DB
    /// - When the owner is not a teacher or admin
    pub async fn add<C: ConnectionTrait + TransactionTrait>(
        db: &C,
        params: &AddParams,
    ) -> ModelResult<Self> {
        use super::users::Role;

        let txn = db.begin().await?;

        if !matches!(params.owner.role, Role::Teacher | Role::Admin) {
            return Err(ModelError::Any(Error::PermissionDenied.into()));
        }

        let description = descriptions::Model::add(&txn, &params.description).await?;

        let problem = ActiveModel {
            name: ActiveValue::set(params.name.to_string()),
            owner_id: ActiveValue::set(params.owner.id),
            r#type: params
                .r#type
                .map_or(ActiveValue::NotSet, |t| ActiveValue::set(t as i32)),
            status: params
                .status
                .map_or(ActiveValue::NotSet, |s| ActiveValue::set(s as i32)),
            description_id: ActiveValue::set(description.id),
            allowed_language: params
                .allowed_language
                .map_or(ActiveValue::NotSet, ActiveValue::set),
            quota: params.quota.map_or(ActiveValue::NotSet, ActiveValue::set),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(transform_db_error)?;

        txn.commit().await.map_err(transform_db_error)?;

        Ok(problem)
    }

    /// List problems
    ///
    /// # Errors
    ///
    /// When cloud not query problems from DB
    pub async fn list<C: ConnectionTrait>(db: &C, params: &ListParams) -> ModelResult<Vec<Self>> {
        // TODO: check course && tags

        let mut q = Problems::find().order_by(problems::Column::Id, Order::Asc);

        if let Some(name) = &params.name {
            q = q.filter(problems::Column::Name.eq(name));
        }

        let problems = q.all(db).await?.into_iter();
        // TODO: permission check

        let offset = params.offset.unwrap_or(0);
        let count = params.count.unwrap_or(usize::MAX);
        let problems = problems.skip(offset).take(count);

        Ok(problems.collect())
    }

    /// Find a problem by its primary id
    ///
    /// # Errors
    ///
    /// - When cloud not query problem from DB
    /// - When the problem with id does not exist
    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i32) -> ModelResult<Self> {
        let p = Problems::find()
            .filter(problems::Column::Id.eq(id))
            .one(db)
            .await?;
        p.ok_or(ModelError::EntityNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serde_visibility() {
        assert_eq!(json!(Visibility::Show), json!(0));
        assert_eq!(json!(Visibility::Hidden), json!(1));
        assert_eq!(
            Visibility::Show,
            serde_json::from_str::<Visibility>("0").unwrap(),
        );
        assert_eq!(
            Visibility::Hidden,
            serde_json::from_str::<Visibility>("1").unwrap(),
        );
        assert!(serde_json::from_str::<Visibility>("3").is_err());
    }
}
