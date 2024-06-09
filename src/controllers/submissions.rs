use format::render;
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::models::submissions::{self, Language, SubmissionStatus};

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

pub struct ListSubmissionRequest {
    pub offset: Option<usize>,
    pub count: Option<usize>,
    pub problem: Option<i32>,
    pub user: Option<i32>,
    pub status: Option<SubmissionStatus>,
    pub language: Option<Language>,
    pub course: Option<String>,
}

async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<ListSubmissionRequest>,
) -> Result<Response> {
    let params = submissions::ListParams {
        offset: params.offset,
        count: params.count,
        problem: params.problem,
        user: params.user,
        status: params.status,
        language: params.language,
        course: params.course,
    };

    let submissions = submissions::Model::list(&ctx.db, &params).await?;
}
