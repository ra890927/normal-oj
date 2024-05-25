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
