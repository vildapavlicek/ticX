pub type DbResult<T> = Result<T, DbError>;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Failed to connect to {uri}. Reason: {error}")]
    ConnectionFailure { uri: String, error: String },
    #[error("Failed to run migrations to set up DB")]
    MigrationFailure(String),
    #[error("Failed to execute query {query}. Reason {error}")]
    QueryExecuteError { query: &'static str, error: String },
    #[error("failed to retrieve connection from connection pool. Reason: {0}")]
    NoConnectionAvailable(String),
    #[error("failed to insert into {table}. Reason: {error}")]
    InsertError { table: &'static str, error: String },
}

impl DbError {
    pub(crate) fn connection_error<T: std::fmt::Display>(uri: &str, err: T) -> Self {
        tracing::error!(%uri, %err, "failed to connect to DB");
        Self::ConnectionFailure {
            uri: uri.to_owned(),
            error: err.to_string(),
        }
    }

    pub(crate) fn migration_failure<T: std::fmt::Display>(err: T) -> Self {
        tracing::error!(%err, "failed to run migrations to set up DB");
        Self::MigrationFailure(err.to_string())
    }

    pub(crate) fn query_error<T: std::fmt::Display>(query: &'static str, err: T) -> Self {
        tracing::error!(%query, %err, "failed to execute query");
        Self::QueryExecuteError {
            query,
            error: err.to_string(),
        }
    }

    pub(crate) fn connection_not_available<T: std::fmt::Display>(
        query: &'static str,
        err: T,
    ) -> Self {
        tracing::error!(%query, %err, "failed to retrieve DB connection to execute query");
        Self::NoConnectionAvailable(err.to_string())
    }

    pub(crate) fn insert_error<T: std::fmt::Display>(table: &'static str, err: T) -> Self {
        tracing::error!(%table, %err, "failed to insert data into table");
        Self::InsertError {
            table,
            error: err.to_string(),
        }
    }
}
