use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Courses::Table)
                    .col(pk_auto(Courses::Id))
                    .col(string_len_uniq(Courses::Name, 64))
                    .col(integer(Courses::TeacherId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-course-teacher")
                            .from(Courses::Table, Courses::TeacherId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Courses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Courses {
    Table,
    Id,
    Name,
    TeacherId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
