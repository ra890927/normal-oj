use eyre::Context;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use normal_oj::{
    app::App,
    models::{
        courses,
        users::{self, RegisterParams},
    },
};
use sea_orm::TryIntoModel;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ctx = playground::<App>().await.context("playground")?;

    // let active_model: articles::ActiveModel = ActiveModel {
    //     title: Set(Some("how to build apps in 3 steps".to_string())),
    //     content: Set(Some("use Loco: https://loco.rs".to_string())),
    //     ..Default::default()
    // };
    // active_model.insert(&ctx.db).await.unwrap();

    // let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    // println!("{:?}", res);
    println!("welcome to playground. edit me at `examples/playground.rs`");

    let u = users::Model::create_with_password(
        &_ctx.db,
        &RegisterParams {
            username: "teacher1".to_string(),
            email: "teacher1@noj.tw".to_string(),
            password: "teacher1".to_string(),
        },
    )
    .await?;
    let mut u = u.into_active_model();
    u.role = ActiveValue::set(users::Role::Teacher);
    let u = u.save(&_ctx.db).await?.try_into_model().unwrap();
    println!("{u:?}");

    let c = courses::ActiveModel {
        name: ActiveValue::Set("course1".to_string()),
        teacher_id: ActiveValue::Set(u.id),
        ..Default::default()
    };
    let c = c.save(&_ctx.db).await?.try_into_model().unwrap();
    println!("{c:?}");

    Ok(())
}
