use sqlx::{MySqlPool, query};
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<MySqlPool> = OnceCell::const_new();

pub async fn get_db_pool() -> &'static MySqlPool {
    dotenv::dotenv().ok();

    DB_POOL.get_or_init(|| async {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MySqlPool::connect(&database_url).await.expect("Failed to create database pool")
    }).await
}

pub async fn insert_into_database(pool: &MySqlPool, generated_value: i32, response_body: &str) -> Result<(), sqlx::Error> {
    query!("INSERT INTO requests (generated_value, response_body) VALUES (?, ?)", generated_value, response_body)
        .execute(pool)
        .await?;

    Ok(())
}
