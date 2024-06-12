use serde::Serialize;

use super::NojResponseBuilder;
use crate::models::submissions::{self, Language, SubmissionStatus};

impl Into<i32> for SubmissionStatus {
    fn into(self) -> i32 {
        match self {
            SubmissionStatus::Pending => -1,
            SubmissionStatus::Accepted => 0,
            SubmissionStatus::WrongAnswer => 1,
            SubmissionStatus::ComileError => 2,
            SubmissionStatus::TimeLimitError => 3,
            SubmissionStatus::MemoryLimitError => 4,
            SubmissionStatus::RuntimeError => 5,
            SubmissionStatus::JudgeError => 6,
            SubmissionStatus::OutputLimitError => 7,
        }
    }
}

impl From<i32> for SubmissionStatus {
    fn from(val: i32) -> Self {
        match val {
            -1 => SubmissionStatus::Pending,
            0 => SubmissionStatus::Accepted,
            1 => SubmissionStatus::WrongAnswer,
            2 => SubmissionStatus::ComileError,
            3 => SubmissionStatus::TimeLimitError,
            4 => SubmissionStatus::MemoryLimitError,
            5 => SubmissionStatus::RuntimeError,
            6 => SubmissionStatus::JudgeError,
            7 => SubmissionStatus::OutputLimitError,
            _ => panic!("error submission type"),
        }
    }
}

impl Into<i32> for Language {
    fn into(self) -> i32 {
        match self {
            Language::C => 0,
            Language::Cpp => 1,
            Language::Python => 2,
        }
    }
}

impl From<i32> for Language {
    fn from(val: i32) -> Self {
        match val {
            0 => Language::C,
            1 => Language::Cpp,
            2 => Language::Python,
            _ => panic!("error language type"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SubmissionListResponseItem {
    pub id: i32,
    pub user_id: i32,
    pub problem_id: i32,
    pub timestamp: i64,
    pub score: i32,
    pub exec_time: i32,
    pub memory_usage: i32,
    pub code: String,
    pub last_send: i64,
    pub status: i32,
    pub language: i32,
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
                problem_id: p.problem_id,
                timestamp: p.timestamp.and_utc().timestamp(),
                score: p.score,
                exec_time: p.exec_time,
                status: p.status.clone().into(),
                language: p.language.clone().into(),
                last_send: p.last_send.and_utc().timestamp(),
                memory_usage: p.memory_usage,
                code: p.code.to_string(),
            })
            .collect();

        NojResponseBuilder::new(data)
    }
}
