use sea_orm_migration::prelude::*;

use super::m20240916_014243_create_author_table::Author;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Author::Table)
                    .add_column(
                        ColumnDef::new(Author::FirstName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column(
                        ColumnDef::new(Author::LastName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Author::Table)
                    .drop_column(Author::FirstName)
                    .drop_column(Author::LastName)
                    .to_owned(),
            )
            .await
    }
}
