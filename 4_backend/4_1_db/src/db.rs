use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log;

pub async fn db_connect() -> DatabaseConnection {

    let mut options = ConnectOptions::new("postgres://postgres:postgres@localhost:5432/postgres");
    options.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(options).await.unwrap();

    db
}