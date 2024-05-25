use loco_rs::schema::string_uniq;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create problem_descriptions table
        manager
            .create_table(
                table_auto(ProblemDescriptions::Table)
                    .col(pk_auto(ProblemDescriptions::Id))
                    .col(string_len(ProblemDescriptions::Description, 1 << 20))
                    .col(string_len(ProblemDescriptions::Input, 1 << 20))
                    .col(string_len(ProblemDescriptions::Output, 1 << 20))
                    .col(string_len(ProblemDescriptions::Hint, 1 << 10))
                    .col(array(ProblemDescriptions::SampleInput, ColumnType::Text))
                    .col(array(ProblemDescriptions::SampleOutput, ColumnType::Text))
                    .to_owned(),
            )
            .await?;

        // create problems table
        manager
            .create_table(
                table_auto(Problems::Table)
                    .col(pk_auto(Problems::Id))
                    .col(string_len(Problems::Name, 1 << 8))
                    .col(integer(Problems::OwnerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-owner")
                            .from(Problems::Table, Problems::OwnerId)
                            .to(Users::Table, Users::Id),
                    )
                    .col(integer(Problems::Type))
                    .col(integer(Problems::Status))
                    .col(integer_uniq(Problems::DescriptionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-problem-description")
                            .from(Problems::Table, Problems::DescriptionId)
                            .to(ProblemDescriptions::Table, ProblemDescriptions::Id),
                    )
                    // allow all by default
                    .col(integer(Problems::AllowedLanguage).default(7))
                    .col(integer(Problems::Quota).default(-1))
                    .col(string_uniq(Problems::TestCaseId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Problems::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProblemDescriptions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProblemDescriptions {
    Table,
    Id,
    Description,
    Input,
    Output,
    Hint,
    SampleInput,
    SampleOutput,
}

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
    Name,
    OwnerId,
    Type,
    Status, // visibility
    DescriptionId,
    AllowedLanguage,
    Quota,
    TestCaseId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
