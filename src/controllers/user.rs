use axum::{extract::Query, http::StatusCode, routing::patch};
use loco_rs::{
    controller::views::pagination::{Pager, PagerMeta},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    models::users::{self, RegisterParams, Role},
    views::user::{CurrentResponse, UserInfoResponse},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserParams {
    role: Option<i32>,
    course: Option<String>,
}

async fn verify_admin(
    ctx: &AppContext,
    auth: &auth::JWT,
) -> Result<users::Model, Result<Response>> {
    let user = users::Model::find_by_claims_key(&ctx.db, &auth.claims.pid)
        .await
        .map_err(|e| Err(e.into()))?;

    if Role::Admin != user.role {
        return Err(format::render()
            .status(StatusCode::FORBIDDEN)
            .json(json!({"msg": "Insufficient Permissions"})));
    }

    return Ok(user);
}

async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let _user = match verify_admin(&ctx, &auth).await {
        Ok(u) => u,
        Err(e) => return e,
    };

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

async fn list_user(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<ListUserParams>,
    Query(page_params): Query<model::query::PaginationQuery>,
) -> Result<Response> {
    let _user = match verify_admin(&ctx, &auth).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    let role = match params.role {
        Some(0) => Some(Role::Admin),
        Some(1) => Some(Role::Teacher),
        Some(2) => Some(Role::Student),
        None => None,
        Some(_) => {
            return format::render()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .json(json!({"msg": "invalid role id"}))
        }
    };

    let condition = role.map(|r| {
        model::query::dsl::condition()
            .eq(users::users::Column::Role, r)
            .build()
    });
    let user_list =
        model::query::exec::paginate(&ctx.db, users::Entity::find(), condition, &page_params)
            .await?;
    let resp = Pager::new(
        user_list
            .rows
            .iter()
            .map(UserInfoResponse::new)
            .collect::<Vec<_>>(),
        PagerMeta {
            page: user_list.info.page,
            page_size: user_list.info.page_size,
            total_pages: user_list.info.total_pages,
        },
    );

    format::json(resp)
}

async fn edit_user(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
    Path(username): Path<String>,
    Json(params): Json<users::EditParams>,
) -> Result<Response> {
    let user = match verify_admin(&ctx, &auth).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    let user_to_edit = users::Model::find_by_username(&ctx.db, &username)
        .await?
        .into_active_model()
        .edit(&ctx.db, params)
        .await?;
    tracing::info!(
        admin = user.name,
        user = user_to_edit.name,
        "user is edited by admin"
    );

    format::json("")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("user")
        .add("/current", get(current))
        .add("", post(create))
        .add("", get(list_user))
        .add("/:username", patch(edit_user))
}
