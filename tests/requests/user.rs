use axum::http::StatusCode;
use insta::{assert_debug_snapshot, assert_json_snapshot, with_settings};
use loco_rs::{app::AppContext, testing};
use normal_oj::{
    app::App,
    models::users::{self, Role},
    views::{user::UserInfoResponse, PaginatedResponse},
};
use rstest::rstest;
use serde_json::json;
use serial_test::serial;

use super::prepare_data;

// TODO: see how to dedup / extract this to app-local test utils
// not to framework, because that would require a runtime dep on insta
macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("user_request");
        let _guard = settings.bind_to_scope();
    };
}

async fn create_token(user: &users::Model, ctx: &AppContext) -> String {
    let jwt_secret = ctx.config.get_jwt_config().unwrap();
    user.generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .unwrap()
}

#[tokio::test]
#[serial]
async fn can_get_current_user() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let response = request
            .get("/api/user/current")
            .add_header(auth_key, auth_value)
            .await;

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!((response.status_code(), response.text()));
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn normal_user_cannot_add_user() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let create_user_payload = json!({
            "username": "new_user",
            "email": "somebody@noj.tw",
            "password": "password",
        });

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let response = request
            .post("/api/user")
            .json(&create_user_payload)
            .add_header(auth_key, auth_value)
            .await;

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!((response.status_code(), response.text()));
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn admin_can_add_user() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        let username = "new_user";
        let password = "password";

        let user = users::Model::find_by_username(&ctx.db, "first_admin")
            .await
            .unwrap();
        let token = create_token(&user, &ctx).await;

        let create_user_payload = json!({
            "username": username,
            "email": "somebody@noj.tw",
            "password": password,
        });

        let (auth_key, auth_value) = prepare_data::auth_header(&token);
        let response = request
            .post("/api/user")
            .json(&create_user_payload)
            .add_header(auth_key, auth_value)
            .await;
        assert_debug_snapshot!((response.status_code(), response.text()));

        let response = request
            .post("/api/auth/login")
            .json(&json!({
                "username": username,
                "password": password,
            }))
            .await;
        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!((response.status_code(), response.text()));
        });
    })
    .await;
}

#[rstest]
#[case("first_admin", 200)]
#[case("user1", 403)]
#[tokio::test]
#[serial]
async fn list_users(#[case] username: &str, #[case] status_code: u16) {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        let user = users::Model::find_by_username(&ctx.db, username)
            .await
            .unwrap();
        let (auth_key, auth_value) = prepare_data::auth_header(&create_token(&user, &ctx).await);
        let response = request
            .get("/api/user")
            .add_header(auth_key, auth_value)
            .await;
        response.assert_status(StatusCode::from_u16(status_code).unwrap());
        with_settings!({
            filters => testing::cleanup_user_model(),
        }, {
            assert_json_snapshot!(
                format!("list_users_{username}"),
                response.json::<serde_json::Value>(),
                {
                    ".results" => insta::sorted_redaction()
                },
            );
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_filter_user_by_role() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        let user = users::Model::find_by_username(&ctx.db, "first_admin")
            .await
            .unwrap();
        let (auth_key, auth_value) = prepare_data::auth_header(&create_token(&user, &ctx).await);
        let response = request
            .get("/api/user")
            .add_header(auth_key, auth_value)
            .add_query_param("role", Role::Admin as i32)
            .await;
        response.assert_status_ok();

        let response = response.json::<PaginatedResponse<UserInfoResponse>>();
        assert!(response
            .results
            .iter()
            .all(|u| Role::Admin as i32 == u.role));

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_json_snapshot!(response);
        });
    })
    .await;
}
