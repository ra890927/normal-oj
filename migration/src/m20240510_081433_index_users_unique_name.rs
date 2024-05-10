use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Users {
    Table,
    Name,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(ColumnDef::new(Users::Name).string().unique_key())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // HACK: hard-coded raw SQL, but I really don't know how to get this work
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "ALTER TABLE users DROP CONSTRAINT users_name_key",
            [],
        ))
        .await?;

        Ok(())
    }
}
