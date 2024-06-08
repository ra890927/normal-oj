use crate::{
    models::{
        self,
        problems::{self, Type, Visibility},
        transform_db_error, users,
    },
    views::problems::{ProblemDetailResponse, ProblemListResponse},
};
use axum::extract::Query;
use loco_rs::{controller::format::render, prelude::*};
use serde::Deserialize;

use super::verify_admin;

#[derive(Debug, Deserialize)]
pub struct CreateProblemRequest {
    /// list of course names
    pub courses: Vec<String>,
    pub name: String,
    /// Problem status, control its visibility
    pub status: Option<Visibility>,
    /// Problem description struct
    pub description: problems::descriptions::AddParams,
    pub r#type: Option<Type>,
    pub allowed_language: Option<i32>,
    pub quota: Option<i32>,
}

async fn create(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
    Json(params): Json<CreateProblemRequest>,
) -> Result<Response> {
    let user = match verify_admin(&ctx, &auth).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    let params = problems::AddParams {
        owner: user,
        courses: params.courses,
        name: params.name,
        status: params.status,
        description: params.description,
        r#type: params.r#type,
        allowed_language: params.allowed_language,
        quota: params.quota,
    };

    let problem = problems::Model::add(&ctx.db, &params).await?;

    render().json(problem)
}

#[derive(Debug, Deserialize)]
pub struct ListProblemRequest {
    pub offset: Option<usize>,
    pub count: Option<usize>,
    pub name: Option<String>,
    pub tags: Option<String>,
    pub course: Option<String>,
}

async fn list(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
    params: Query<ListProblemRequest>,
) -> Result<Response> {
    let user = users::Model::find_by_claims_key(&ctx.db, &auth.claims.pid).await?;

    let params = problems::ListParams {
        viewer: user,
        offset: params.offset,
        count: params.count,
        name: params.name.clone(),
        tags: params
            .tags
            .as_ref()
            .map(|t| t.split(',').map(str::to_string).collect()),
        course: params.course.clone(),
    };

    let problems = problems::Model::list(&ctx.db, &params).await?;

    format::json(ProblemListResponse::new(&problems).done())
}

async fn get_problem(
    State(ctx): State<AppContext>,
    // auth: auth::JWT,
    Path(problem_id): Path<i32>,
) -> Result<Response> {
    // TOOD: authz

    let prob = problems::Model::find_by_id(&ctx.db, problem_id).await?;
    let desc = prob
        .find_related(models::_entities::problem_descriptions::Entity)
        .one(&ctx.db)
        .await
        .map_err(|err| transform_db_error(err))?
        .ok_or(ModelError::EntityNotFound)?;
    let owner = prob
        .find_related(models::_entities::users::Entity)
        .one(&ctx.db)
        .await
        .map_err(|err| transform_db_error(err))?
        .ok_or(ModelError::EntityNotFound)?;

    format::json(ProblemDetailResponse::new(&prob, &desc, &owner).done())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("problems")
        .add("/", post(create))
        .add("/manage", post(create))
        .add("/", get(list))
        .add("/:problem_id", get(get_problem))
        .add("/view/:problem_id", get(get_problem))
}
