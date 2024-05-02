#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240430_100035_add_users_role;
mod m20240502_121640_add_users_displayed_name;
mod m20240502_122830_add_users_bio;
mod m20240502_130956_courses;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240430_100035_add_users_role::Migration),
            Box::new(m20240502_121640_add_users_displayed_name::Migration),
            Box::new(m20240502_122830_add_users_bio::Migration),
            Box::new(m20240502_130956_courses::Migration),
        ]
    }
}
