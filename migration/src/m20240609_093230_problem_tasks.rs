use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ProblemTasks {
    Table,
    Id,
    TestCaseCount,
    Score,
    TimeLimit,
    MemoryLimit,
    ProblemId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProblemTasks::Table)
                    .col(pk_auto(ProblemTasks::Id))
                    .col(integer(ProblemTasks::TestCaseCount))
                    .col(integer(ProblemTasks::Score))
                    .col(integer(ProblemTasks::TimeLimit))
                    .col(integer(ProblemTasks::MemoryLimit))
                    .col(integer(ProblemTasks::ProblemId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-task-problem")
                            .from(ProblemTasks::Table, ProblemTasks::ProblemId)
                            .to(Problems::Table, Problems::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProblemTasks::Table).to_owned())
            .await?;

        Ok(())
    }
}
