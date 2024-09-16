use std::env::{var, VarError};

pub struct DbConfig {
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
}

impl DbConfig {
    pub fn new() -> Result<Self, VarError> {
        Ok(Self {
            db_host: var("DB_HOST")?,
            db_port: var("DB_PORT")?,
            db_name: var("DB_NAME")?,
            db_user: var("DB_USER")?,
            db_password: var("DB_PASSWORD")?,
        })
    }
}
