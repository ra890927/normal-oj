use crate::models::courses;
use loco_rs::prelude::*;

pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(courses::Entity::find().all(&ctx.db).await?)
}

pub async fn get_one(Path(name): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(courses::Model::find_by_name(&ctx.db, &name).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("courses")
        .add("/", get(list))
        .add("/:name", get(get_one))
}
