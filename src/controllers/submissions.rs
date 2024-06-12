use axum::extract::Query;
use format::render;
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::{
    models::submissions::{self, Language},
    views::submission::SubmissionListResponse,
};

#[derive(Debug, Deserialize)]
pub struct CreateSubmissionRequest {
    pub user: i32,
    pub problem: i32,
    pub timestamp: DateTime,
    pub language: Language,
}

async fn create(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateSubmissionRequest>,
) -> Result<Response> {
    let params = submissions::AddParams {
        user: params.user,
        problem: params.problem,
        timestamp: params.timestamp,
        language: params.language,
    };

    let submission = submissions::Model::add(&ctx.db, &params).await?;

    render().json(submission)
}

#[derive(Debug, Deserialize)]
pub struct ListSubmissionRequest {
    pub offset: Option<usize>,
    pub count: Option<usize>,
    pub problem: Option<i32>,
    pub user: Option<i32>,
    pub status: Option<i32>,
    pub language: Option<i32>,
    pub course: Option<String>,
}

async fn list(
    State(ctx): State<AppContext>,
    params: Query<ListSubmissionRequest>,
) -> Result<Response> {
    let params = submissions::ListParams {
        offset: params.offset,
        count: params.count,
        problem: params.problem,
        user: params.user,
        status: params.status.map(|s| s.into()),
        language: params.language.map(|l| l.into()),
        course: params.course.clone(),
    };

    let submissions = submissions::Model::list(&ctx.db, &params).await?;

    format::json(SubmissionListResponse::new(&submissions).done())
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubmissionResultRequest {
    pub score: i32,
    pub exec_time: i32,
    pub memory_usage: i32,
}

async fn update_sandbox_result(
    State(ctx): State<AppContext>,
    Path(submission_id): Path<i32>,
    Json(params): Json<UpdateSubmissionResultRequest>,
) -> Result<Response> {
    let submission = submissions::Model::find_by_id(&ctx.db, submission_id).await?;
    submission
        .into_active_model()
        .update_sandbox_result(&ctx.db, params.score, params.exec_time, params.memory_usage)
        .await?;

    format::empty_json()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("submissions")
        .add("/", get(list))
        .add("/", post(create))
        .add("/:submission_id", put(update_sandbox_result))
}
