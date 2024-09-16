mod config;
mod migrator;

use config::db_connect;
use migrator::Migrator;
use sea_orm_migration::MigratorTrait;
use std::process;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello, world!")
}

#[launch]
async fn rocket() -> _ {
    let db_conn = match db_connect().await {
        Ok(conn) => conn,
        Err(_) => {
            process::exit(1);
        }
    };

    match Migrator::up(&db_conn, None).await {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => {
            eprintln!("Failed to run migrations: {}", e);
            process::exit(1);
        }
    };

    rocket::build()
        .manage(db_conn)
        .mount("/api", routes![hello])
}
