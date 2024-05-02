use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden, EnumIter)]
enum Role {
    Admin,
    Teacher,
    Student,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Role,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let role_name = || Alias::new("role");

        // Add role type
        manager
            .create_type(
                Type::create()
                    .as_enum(role_name())
                    .values(Role::iter())
                    .to_owned(),
            )
            .await?;
        // Add role column
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Users::Role)
                            .enumeration(role_name(), Role::iter())
                            .default(Role::Student.to_string())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let role_name = || Alias::new("role");

        // drop column first, or we cannot drop type because user.role depends on it
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Role)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(role_name()).to_owned())
            .await?;

        Ok(())
    }
}
