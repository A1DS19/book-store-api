pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20240916_014243_create_author_table;
mod m20240916_015113_create_book_table;
mod m20240918_202150_add_first_name_and_last_name_author;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20240916_014243_create_author_table::Migration),
            Box::new(m20240916_015113_create_book_table::Migration),
            Box::new(m20240918_202150_add_first_name_and_last_name_author::Migration),
        ]
    }
}
