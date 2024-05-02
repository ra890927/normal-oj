pub mod auth;
pub mod user;

use loco_rs::controller::views::pagination::PagerMeta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub results: Vec<T>,
    pub pagination: PagerMeta,
}
