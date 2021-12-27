pub use sqlx;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};

use std::env;
use std::lazy::SyncOnceCell;

pub static DATABASE_POOL: SyncOnceCell<Pool<Postgres>> = SyncOnceCell::new();

pub async fn connect() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::new()
                .username("postgres")
                .password(
                    env::var("DATABASE_PASSWORD")
                        .expect("DATABASE_PASSWORD environment variable not set")
                        .as_str(),
                )
                .database("keymaster"),
        )
        .await
        .expect("Failed to connect to database.");

    DATABASE_POOL
        .set(pool.clone())
        .expect("Failed to set database pool.");

    pool
}

pub async fn migrate(db: &Pool<Postgres>) {
    let migrator: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
    migrator.run(db).await.expect("Failed to run migrations.");
}

pub async fn start() {
    let pool = connect().await;
    migrate(&pool).await;
}
