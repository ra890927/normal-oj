use crate::models::problems::{self, Type, Visibility};
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("problems")
        .add("/", post(create))
        .add("/manage", post(create))
}
