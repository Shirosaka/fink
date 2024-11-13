//! # Database models and utilities

use anyhow::Result;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub type PostgresPool = Pool<AsyncPgConnection>;

pub fn init_pool(url: String, size: usize) -> Result<PostgresPool> {
    tracing::info!(url = ?url, "Connecting to database");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    let pool = Pool::builder(manager).max_size(size).build()?;

    Ok(pool)
}
