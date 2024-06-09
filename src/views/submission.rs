use serde::Serialize;

use super::Noj
use crate::models::submissions::{self, Language, SubmissionStatus};

#[derive(Debug, Serialize)]
pub struct SubmissionListResponseItem {
    pub id: i32,
    pub user_id: i32,
    pub score: i32,
    pub problem_id: i32,
    pub timestamp: f64,
    pub status: SubmissionStatus,
    pub language: Language,
}

pub struct SubmissionListResponse {}

impl SubmissionListResponse {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        submissions: &[submissions::Model],
    ) -> NojResponseBuilder<Vec<SubmissionListResponseItem>> {
        let data = submissions
            .iter()
            .map(|p| SubmissionListResponseItem {
                id: p.id,
                user_id: p.user_id,
                score: p.score,
                problem_id: p.problem_id,
                timestamp: p.timestamp,
                status: p.status,
                language: p.language,
            })
            .collect();

        NojResponseBuilder::new(data)
    }
}
