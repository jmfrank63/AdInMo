use dotenv::dotenv;
use sqlx::{query, MySqlPool};
use tokio::sync::OnceCell;

static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

pub async fn initialize_db_pool() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    POOL.set(pool).expect("Failed to set database pool");
    Ok(())
}

pub async fn get_db_pool() -> &'static MySqlPool {
    POOL.get().expect("Database pool has not been initialized")
}

pub async fn insert_into_database(
    generated_value: i32,
    response_body: &str,
) -> Result<(), sqlx::Error> {
    let pool = POOL.get().expect("Database pool not initialized");
    let result = query("INSERT INTO requests (generated_value, response_body) VALUES (?, ?)")
        .bind(generated_value)
        .bind(response_body)
        .execute(pool)
        .await;

    match result {
        Ok(_) => log::trace!("Insert of {} successful", generated_value),
        Err(e) => log::error!("Error executing query: {:?}", e),
    }

    Ok(())
}
