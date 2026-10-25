use sqlx::{Error, PgPool, Pool, Postgres};

pub async fn connect_db(database_url: &str) -> Result<Pool<Postgres>, Error> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    println!("[INFO] Database migrations completed successfully.");
    Ok(())
}
