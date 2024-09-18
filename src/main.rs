mod config;
mod dto;
mod entities;
mod fairings;
mod guards;
mod migrator;
mod routes;

use config::db_connect;
use fairings::cors::CORS;
use migrator::Migrator;
use rocket::http::Status;
use routes::authors::author_books;
use routes::{auth, authors, books};
use routes::{Response, SuccessResponse};
use sea_orm_migration::MigratorTrait;
use std::process;

#[macro_use]
extern crate rocket;

#[get("/health_check")]
fn health_check() -> Response<&'static str> {
    Ok(SuccessResponse((Status::Ok, "Healthy")))
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let config = match config::app_config::AppConfig::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

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
        .attach(CORS)
        .manage(db_conn)
        .manage(config)
        .mount("/api", routes![health_check])
        .mount("/api/auth", routes![auth::sign_in, auth::sign_up, auth::me])
        .mount(
            "/api/authors",
            routes![
                authors::index,
                authors::create,
                authors::show,
                authors::update,
                authors::delete,
                authors::author_books
            ],
        )
        .mount(
            "/api/books",
            routes![
                books::index,
                books::create,
                books::show,
                books::update,
                books::delete
            ],
        )
}
