pub mod _entities;
pub mod courses;
pub mod notes;
pub mod users;

use sea_orm::{DbErr, SqlErr};

pub(crate) fn is_unique_constraint_violation_err(e: &DbErr) -> bool {
    matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_)))
}
pub mod submissions;
