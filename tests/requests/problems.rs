use axum::http::StatusCode;
use insta::{assert_debug_snapshot, assert_json_snapshot, with_settings};
use loco_rs::{app::AppContext, testing};
use normal_oj::{
    app::App,
    models::{
        problems::{self, Type, Visibility},
        users::{self, Role},
    },
    views::{user::UserInfoResponse, PaginatedResponse},
};
use rstest::rstest;
use serde_json::json;
use serial_test::serial;

use super::{create_token, prepare_data};

macro_rules! configure_insta {
    () => {
        crate::configure_insta!("problem_request");
    };
}

fn create_problem_payload() -> serde_json::Value {
    json!({
        "courses": [],
        "name": "A + B",
        "status": 1,
        "description": {
            "description": "",
            "input": "two space-separated number as A & B",
            "output": "A + B",
            "hint": "use +",
            "sample_input": ["1 2"],
            "sample_output": ["3"],
        },
    })
}

#[tokio::test]
#[serial]
async fn student_cannot_create_problem() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let response = request
            .post("/api/problems")
            .add_header(auth_key, auth_value)
            .json(&create_problem_payload())
            .await;

        println!("{response:?}");
        response.assert_status_forbidden();

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!(response.json::<serde_json::Value>());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn view_single_problem() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        let user = prepare_data::init_user_login(&request, &ctx).await;
        let first_admin = users::Model::find_by_username(&ctx.db, "first_admin")
            .await
            .unwrap();
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let problem = problems::Model::add(
            &ctx.db,
            &problems::AddParams {
                owner: first_admin,
                courses: vec![],
                name: "test-course".to_string(),
                status: Some(Visibility::Show),
                description: problems::descriptions::AddParams {
                    description: "".to_string(),
                    input: "".to_string(),
                    output: "".to_string(),
                    hint: "".to_string(),
                    sample_input: vec![],
                    sample_output: vec![],
                },
                r#type: Some(Type::Normal),
                allowed_language: None,
                quota: None,
            },
        )
        .await
        .unwrap();
        let response = request
            .get(&format!("/api/problems/{}", problem.id))
            .add_header(auth_key, auth_value)
            .await;
        response.assert_status_ok();

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!(response.json::<serde_json::Value>());
        });
    })
    .await;
}
