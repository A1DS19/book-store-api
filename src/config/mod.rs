pub mod app_config;

use crate::config::app_config::AppConfig;
use sea_orm::*;

pub(crate) async fn db_connect() -> Result<DatabaseConnection, DbErr> {
    let config = AppConfig::new().map_err(|e| {
        eprintln!("Configuration Error: {}", e);

        DbErr::Conn(RuntimeErr::Internal(
            "Failed to load database configuration".to_string(),
        ))
    })?;

    let mut options = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    ));

    options.sqlx_logging(true);

    let connection = Database::connect(options).await;

    println!("Connected to database");

    connection
}
