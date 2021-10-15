pub type DbResult<T> = Result<T, DbError>;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Failed to connect to {uri}. Reason: {error}")]
    ConnectionFailure { uri: String, error: String },
    #[error("Failed to run migrations to set up DB")]
    MigrationFailure(String),
}

impl DbError {
    pub fn connection_error<T: std::fmt::Display>(uri: &str, err: T) -> Self {
        tracing::error!(%uri, %err, "failed to connect to DB");
        Self::ConnectionFailure {
            uri: uri.to_owned(),
            error: err.to_string(),
        }
    }

    pub fn migration_failure<T: std::fmt::Display>(err: T) -> Self {
        tracing::error!(%err, "failed to run migrations to set up DB");
        Self::MigrationFailure(err.to_string())
    }
}
