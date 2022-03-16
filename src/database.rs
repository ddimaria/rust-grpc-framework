use crate::config::CONFIG;
use crate::error::{Error, Result};
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};

// TODO: support multiple databases
pub(crate) type DatabasePool = Pool<Postgres>;

/// Create a connection pool
pub(crate) async fn pool() -> Result<DatabasePool> {
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .connect(&CONFIG.database_url)
        .await
        .map_err(|error| Error::DatabasePool(error.to_string()))?;

    Ok(pool)
}
