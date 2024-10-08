use sea_orm_migration::{prelude::*, schema::*};

use super::{
    m20220101_000001_create_user_table::User, m20240916_014243_create_author_table::Author,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Book::Id)
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .integer(),
                    )
                    .col(string(Book::Title).not_null())
                    .col(string(Book::Cover).not_null())
                    .col(integer(Book::Year).not_null())
                    .col(integer(Book::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-user_id")
                            .from(Book::Table, Book::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(integer(Book::AuthorId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-author_id")
                            .from(Book::Table, Book::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .col(
                        ColumnDef::new(Book::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Book::UpdatedAt)
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
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    UserId,
    AuthorId,
    Title,
    Year,
    Cover,
    CreatedAt,
    UpdatedAt,
}
