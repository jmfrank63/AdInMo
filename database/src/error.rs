pub enum DatabaseError {
    ConnectionError,
    QueryError,
    NotFound,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            sqlx::Error::PoolTimedOut => DatabaseError::ConnectionError,
            _ => DatabaseError::QueryError,
        }
    }
}
