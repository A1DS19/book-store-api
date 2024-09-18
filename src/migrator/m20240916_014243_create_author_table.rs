use crate::migrator::m20220101_000001_create_user_table::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Author::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .integer(),
                    )
                    .col(integer(Author::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-author-user_id")
                            .from(Author::Table, Author::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(text(Author::Biography).not_null())
                    .col(
                        ColumnDef::new(Author::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Author::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Author {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
    Biography,
    CreatedAt,
    UpdatedAt,
}
