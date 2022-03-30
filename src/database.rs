use crate::config::CONFIG;
use crate::error::{Error, Result};
use sqlx::Pool;

#[cfg(any(feature = "postgres", feature = "cockroachdb"))]
use sqlx::postgres::{PgPoolOptions as DbOptions, Postgres as Db};

#[cfg(feature = "mysql")]
use sqlx::mysql::{MySQL as Db, MySqlPoolOptions as DbOptions};

#[cfg(feature = "sqlite")]
use sqlx::sqlite::{Sqlite as Db, SqlitePoolOptions as DbOptions};

pub(crate) type DatabasePool = Pool<Db>;

/// Create a connection pool
pub(crate) async fn pool() -> Result<DatabasePool> {
    let pool = DbOptions::new()
        .min_connections(5)
        .connect(&CONFIG.database_url)
        .await
        .map_err(|error| Error::DatabasePool(error.to_string()))?;

    Ok(pool)
}
