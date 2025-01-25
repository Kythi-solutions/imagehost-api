use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn create_pool(db_url: String) -> Result<DatabaseConnection, DbErr> {
    let mut conn_opts = ConnectOptions::new(db_url);

    conn_opts
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(conn_opts).await
}
