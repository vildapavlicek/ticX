use actix_web::error::BlockingError;
use actix_web::http::StatusCode;

pub type TicxResult<T> = Result<T, TicxError>;

#[derive(Debug, thiserror::Error)]
pub enum TicxError {
    #[error("Request is missing Authentication Header")]
    MissingAuthHeader,
    #[error("Invalid header '{header}' value '{value}'. Reason: {error}")]
    InvalidHeader {
        header: &'static str,
        value: String,
        error: String,
    },
    #[error("Parsed JWT token is NOT valid. Reason: {0}")]
    InvalidToken(String),
    #[error("Provided invalid credentials")]
    InvalidCredentials,
    #[error("Something really strange or unexpected has happened")]
    Unknown,
    #[error("Failed to execute DB Query properly. Reason: {0}")]
    DbFail(String),
    #[error("failed {what}. Reason: {error}")]
    GenericError { what: &'static str, error: String },
    #[error("requested data not found. error: {0}")]
    NotFound(String),
}

// this shows error because it cannot identify std::fmt::Display being derived
impl actix_web::error::ResponseError for TicxError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingAuthHeader => StatusCode::BAD_REQUEST,
            Self::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::NOT_FOUND,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

use db::errors::DbError;
impl From<DbError> for TicxError {
    fn from(db_error: DbError) -> Self {
        match db_error {
            DbError::InvalidResult => Self::Unknown,
            DbError::InsertError { .. }
            | DbError::UpdateError { .. }
            | DbError::QueryExecuteError { .. } => Self::DbFail(db_error.to_string()),
            // DbError::NoConnectionAvailable(_) => (),
            DbError::NotFound(_) => TicxError::NotFound(db_error.to_string()),
            _ => Self::Unknown,
        }
    }
}

impl From<BlockingError<DbError>> for TicxError {
    fn from(b_err: BlockingError<DbError>) -> Self {
        match b_err {
            BlockingError::Error(db_err) => db_err.into(),
            BlockingError::Canceled => TicxError::Unknown,
        }
    }
}
