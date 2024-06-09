use num_traits::FromPrimitive;
use serde::Serialize;

use crate::models::{
    problems::{self, Type, Visibility},
    users,
};

use super::NojResponseBuilder;

#[derive(Debug, Serialize)]
pub struct ProblemListResponseItem {
    pub id: i32,
    pub name: String,
    pub status: Visibility,
    pub ac_user: i32,
    pub submitter: i32,
    pub tags: Vec<String>,
    pub r#type: Type,
    pub quota: i32,
    pub submit_count: i32,
}

pub struct ProblemListResponse {}

impl ProblemListResponse {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(problems: &[problems::Model]) -> NojResponseBuilder<Vec<ProblemListResponseItem>> {
        let data = problems
            .iter()
            .map(|p| ProblemListResponseItem {
                id: p.id,
                name: p.name.to_string(),
                status: Visibility::from_i32(p.status).unwrap(),
                r#type: Type::from_i32(p.r#type).unwrap(),
                quota: p.quota,
                // TODO: impl following fields
                ac_user: 0,
                submit_count: 0,
                submitter: 0,
                tags: vec![],
            })
            .collect();

        NojResponseBuilder::new(data)
    }
}

#[derive(Debug, Serialize)]
pub struct ProblemDetailResponse {
    // TODO: add fields
    problem_name: String,
    description: problems::descriptions::Model,
    /// username of problem owner
    owner: String,
    tags: Vec<String>,
    allowed_language: i32,
    /// list of courses' names
    courses: Vec<String>,
    quota: i32,
    status: Visibility,
    r#type: Type,
    test_case: Vec<problems::tasks::Model>,
    submit_count: i32,
    high_score: i32,
}

impl ProblemDetailResponse {
    #[must_use]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        problem: &problems::Model,
        description: &problems::descriptions::Model,
        owner: &users::Model,
        tasks: &[problems::tasks::Model],
    ) -> NojResponseBuilder<Self> {
        let resp = Self {
            problem_name: problem.name.clone(),
            description: description.clone(),
            owner: owner.name.clone(),
            tags: vec![],
            allowed_language: problem.allowed_language,
            courses: vec![],
            quota: problem.quota,
            status: Visibility::from_i32(problem.status).unwrap(),
            r#type: Type::from_i32(problem.r#type).unwrap(),
            test_case: tasks.to_vec(),
            submit_count: 0,
            high_score: 0,
        };
        NojResponseBuilder::new(resp)
    }
}
