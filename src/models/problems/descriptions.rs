use crate::models::is_unique_constraint_violation_err;

pub use super::_entities::problem_descriptions::{ActiveModel, Model};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::Deserialize;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

#[derive(Debug, Deserialize)]
pub struct AddParams {
    pub description: String,
    pub input: String,
    pub output: String,
    pub hint: String,
    pub sample_input: Vec<String>,
    pub sample_output: Vec<String>,
}

impl Model {
    /// Create a problem without test case binary
    ///
    /// # Errors
    ///
    /// When could not save the problem into DB
    pub async fn add<C: ConnectionTrait>(db: &C, params: &AddParams) -> ModelResult<Self> {
        let problem_description = ActiveModel {
            description: ActiveValue::set(params.description.to_string()),
            input: ActiveValue::set(params.input.to_string()),
            output: ActiveValue::set(params.output.to_string()),
            hint: ActiveValue::set(params.hint.to_string()),
            sample_input: ActiveValue::set(params.sample_input.clone()),
            sample_output: ActiveValue::set(params.sample_output.clone()),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation_err(&e) {
                ModelError::EntityAlreadyExists {}
            } else {
                ModelError::Any(e.into())
            }
        })?;

        Ok(problem_description)
    }
}
