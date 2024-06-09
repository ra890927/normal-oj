use crate::models::transform_db_error;

pub use super::_entities::problem_tasks::{ActiveModel, Model};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue, IntoActiveModel, TransactionTrait};
use serde::Deserialize;

#[derive(Debug, Deserialize, DeriveIntoActiveModel, Clone)]
pub struct AddParams {
    pub test_case_count: i32,
    pub score: i32,
    pub time_limit: i32,
    pub memory_limit: i32,
}

impl Model {
    pub async fn add_many<C: ConnectionTrait + TransactionTrait>(
        db: &C,
        problem_id: i32,
        params: &[AddParams],
    ) -> ModelResult<Vec<Self>> {
        let txn = db.begin().await?;
        let mut tasks = vec![];
        for p in params {
            let mut task = p.clone().into_active_model();
            task.problem_id = ActiveValue::set(problem_id);
            let task = task.insert(&txn).await.map_err(transform_db_error)?;
            tasks.push(task);
        }
        txn.commit().await?;
        Ok(tasks)
    }
}

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
