use base64::{Engine as _, engine::general_purpose};
use sqlx::Row;
use crate::get_db_pool;
use std::str;

pub async fn authenticate_user(encoded_credentials: &str) -> bool {
    let decoded = general_purpose::STANDARD_NO_PAD.decode(encoded_credentials).unwrap_or_default();
    let credentials = str::from_utf8(&decoded).unwrap_or("");
    let parts: Vec<&str> = credentials.split(':').collect();

    if parts.len() != 2 {
        return false;
    }

    let username = parts[0];
    let password = parts[1];

    // Here, you would verify the username and password against the database
    // For example:
    let pool = get_db_pool().await;
    let sql = "SELECT EXISTS(SELECT 1 FROM users WHERE username = ? AND password = ?)";
    let result = sqlx::query(sql)
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => row.get::<bool, _>(0),
        Err(_) => false,
    }
}
