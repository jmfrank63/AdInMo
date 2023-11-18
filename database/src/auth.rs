use crate::get_root_db_pool;
use base64::{engine::general_purpose, Engine as _};
use sqlx::Row;
use std::str;

pub async fn authenticate_user(encoded_credentials: &str) -> bool {
    log::debug!("Encoded credentials: {}", encoded_credentials);
    let decoded = &general_purpose::URL_SAFE
        .decode(encoded_credentials)
        .unwrap_or_default();
    let credentials = str::from_utf8(decoded).unwrap_or("");
    let parts: Vec<&str> = credentials.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let username = parts[0];
    let password = parts[1];

    let pool = get_root_db_pool().await;
    let sql = "SELECT EXISTS(SELECT 1 FROM mysql.user WHERE User = ? AND Password = PASSWORD(?))";
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
