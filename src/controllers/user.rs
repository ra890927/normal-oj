use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde_json::json;

use crate::{
    models::{users, users::RegisterParams, users::Role},
    views::user::CurrentResponse,
};

async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let user = users::Model::find_by_claims_key(&ctx.db, &auth.claims.pid).await?;

    if Role::Admin != user.role {
        return format::render()
            .status(StatusCode::FORBIDDEN)
            .json(json!({"msg": "Insufficient Permissions"}));
    }

    let new_user = match users::Model::create_with_password(&ctx.db, &params).await {
        Ok(u) => u,
        Err(ModelError::EntityAlreadyExists) => {
            return format::render()
                .status(StatusCode::CONFLICT)
                .json(json!({"msg": "User exists"}));
        }
        Err(ModelError::ModelValidation { errors }) => {
            return format::render()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .json(json!({"msg": "Signup faield", "data": errors }));
        }
        Err(e) => {
            tracing::info!(message = e.to_string(), "could not register user");
            return format::render()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .empty();
        }
    };

    let new_user = new_user.into_active_model().verified(&ctx.db).await?;
    tracing::info!(
        pid = new_user.pid.to_string(),
        "user verified in create user API"
    );

    format::render().status(StatusCode::CREATED).empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("user")
        .add("/current", get(current))
        .add("", post(create))
}
