use std::str::FromStr;

use sqlx::{
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
    SqlitePool,
};

use crate::config::DatabaseSettings;

pub async fn connect(
    settings: &DatabaseSettings,
) -> Result<SqlitePool, sqlx::Error> {
    let options =
        SqliteConnectOptions::from_str(&settings.url)?
            .create_if_missing(true)
            .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(settings.max_connections)
        .connect_with(options)
        .await?;

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;

    Ok(pool)
}