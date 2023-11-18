use crate::{error::DatabaseError, get_db_pool};
use sqlx::query;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct Request {
    pub id: i32,
    pub generated_value: i32,
    pub response_body: String,
}

pub async fn create_request(request: &Request) -> Result<(), DatabaseError> {
    let pool = get_db_pool().await;
    let result = query("INSERT INTO requests (id, value, response_body) VALUES (?, ?, ?)")
        .bind(request.id)
        .bind(request.generated_value)
        .bind(&request.response_body)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::from(e)),
    }
}

// Read
pub async fn get_request(id: i32) -> Result<Request, DatabaseError> {
    let pool = get_db_pool().await;
    let result = sqlx::query_as::<_, Request>("SELECT * FROM requests WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await;

    match result {
        Ok(request) => Ok(request),
        Err(e) => Err(DatabaseError::from(e)),
    }
}

// Update
pub async fn update_request(request: &Request) -> Result<(), DatabaseError> {
    let pool = get_db_pool().await;
    let result =
        sqlx::query("UPDATE requests SET generated_value = ?, response_body = ? WHERE id = ?")
            .bind(request.generated_value)
            .bind(&request.response_body)
            .bind(request.id)
            .execute(pool)
            .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::from(e)),
    }
}

// Delete
pub async fn delete_request(id: i32) -> Result<(), DatabaseError> {
    let pool = get_db_pool().await;
    let result = sqlx::query("DELETE FROM requests WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(DatabaseError::from(e)),
    }
}
