use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(database_url.to_string());
    options
        .max_connections(20)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800));

    Database::connect(options).await
}
