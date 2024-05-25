pub mod auth;
pub mod courses;
pub mod notes;
pub mod problems;
pub mod user;

/// utils
use axum::http::StatusCode;
use loco_rs::{controller::middleware, prelude::*};
use serde_json::json;

use crate::models::users::{self, Role};

async fn verify_admin(
    ctx: &AppContext,
    auth: &middleware::auth::JWT,
) -> Result<users::Model, Result<Response>> {
    let user = users::Model::find_by_claims_key(&ctx.db, &auth.claims.pid)
        .await
        .map_err(|e| Err(e.into()))?;

    if Role::Admin != user.role {
        return Err(format::render()
            .status(StatusCode::FORBIDDEN)
            .json(json!({"msg": "Insufficient Permissions"})));
    }

    Ok(user)
}
