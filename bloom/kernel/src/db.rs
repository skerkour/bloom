use crate::config;
use sqlx::{postgres::PgPoolOptions, Executor, Pool, Postgres, Transaction};
use std::time::Duration;
use stdx::log::error;

pub type DB = Pool<Postgres>;
pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}

impl<'c> Queryer<'c> for &Pool<Postgres> {}
impl<'c> Queryer<'c> for &'c mut Transaction<'_, Postgres> {}

pub async fn connect(database: &config::Database) -> Result<DB, crate::Error> {
    // See https://www.alexedwards.net/blog/configuring-sqldb
    // and https://making.pusher.com/production-ready-connection-pooling-in-go
    // for the details
    // ret.SetMaxOpenConns(int(poolSize))
    // ret.SetMaxIdleConns(int(poolSize / 2))
    // ret.SetConnMaxLifetime(30 * time.Minute)
    PgPoolOptions::new()
        .max_connections(database.pool_size)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(&database.url)
        .await
        .map_err(|err| {
            error!("db: connecting to DB: {}", err);
            err.into()
        })
}

pub async fn migrate(db: &DB) -> Result<(), crate::Error> {
    sqlx::migrate!("../db/migrations").run(db).await?;

    Ok(())
}
