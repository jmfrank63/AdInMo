use crate::{error::DatabaseError, get_db_pool};
use sqlx::query;

#[derive(sqlx::FromRow)]
pub struct Record {
    pub id: i32,
    pub value: i32,
    pub response_body: String,
}

pub async fn create_record(record: &Record) -> Result<(), DatabaseError> {
    let pool = get_db_pool().await;
    let result = query("INSERT INTO records (id, value, response_body) VALUES (?, ?, ?)")
        .bind(record.id)
        .bind(record.value)
        .bind(&record.response_body)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::from(e)),
    }
}

// Read
pub async fn get_record(id: i32) -> Result<Record, DatabaseError> {
    let pool = get_db_pool().await;
    let result = sqlx::query_as::<_, Record>("SELECT * FROM records WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await;

    match result {
        Ok(record) => Ok(record),
        Err(e) => Err(DatabaseError::from(e)),
    }
}

// Update
pub async fn update_record(record: &Record) -> Result<(), DatabaseError> {
    let pool = get_db_pool().await;
    let result = sqlx::query("UPDATE requests SET generated_value = ?, response_body = ? WHERE id = ?")
        .bind(record.value)
        .bind(&record.response_body)
        .bind(record.id)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::from(e)),
    }
}

// Delete
pub async fn delete_record(id: i32) -> Result<(), DatabaseError> {
    let pool = get_db_pool().await;
    let result = sqlx::query("DELETE FROM records WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::from(e)),
    }
}
