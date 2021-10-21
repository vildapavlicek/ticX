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
    #[error("failed to update {what}. Reason: {error}")]
    UpdateError { what: &'static str, error: String },
    #[error("Requested resource '{0}' not found in database")]
    NotFound(&'static str),
}

impl DbError {
    pub(crate) fn connection_error<T: std::fmt::Display>(uri: &str, err: T) -> Self {
        tracing::error!(%uri, %err, "failed to connect to DB");
        Self::ConnectionFailure {
            uri: uri.to_owned(),
            error: err.to_string(),
        }
    }

    /*     pub(crate) fn migration_failure<T: std::fmt::Display>(err: T) -> Self {
           tracing::error!(%err, "failed to run migrations to set up DB");
           Self::MigrationFailure(err.to_string())
       }
    */

    pub(crate) fn resolve_diesel_error(err: diesel::result::Error, action: &'static str) {
        match err {
            diesel::result::Error::InvalidCString(_) => (),
            diesel::result::Error::DatabaseError(_, _) => (),
            diesel::result::Error::NotFound => (),
            diesel::result::Error::QueryBuilderError(_) => (),
            diesel::result::Error::DeserializationError(_) => (),
            diesel::result::Error::SerializationError(_) => (),
            diesel::result::Error::RollbackTransaction => (),
            diesel::result::Error::AlreadyInTransaction => (),
            _ => (),
        }
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

    pub(crate) fn update_error<T: std::fmt::Display>(what: &'static str, err: T) -> Self {
        tracing::error!(%what, %err, "failed to update data");
        Self::UpdateError {
            what,
            error: err.to_string(),
        }
    }

    pub(crate) fn not_found(what: &'static str) -> Self {
        tracing::error!(%what, "requested resource not found in DB");
        Self::NotFound(what)
    }
}
