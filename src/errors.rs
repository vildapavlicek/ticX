use actix_web::http::StatusCode;
use std::fmt::{Display, Formatter};

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
}

// this shows error because it cannot identify std::fmt::Display being derived
impl actix_web::error::ResponseError for TicxError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingAuthHeader => StatusCode::BAD_REQUEST,
            Self::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// use db::errors::DbError;
// impl From<DbError> for TicxError {
//     fn from(db_error: DbError) -> Self {
//         match db_error {
//             DbError::InvalidResult => Self::Unknown,
//             DbError::InsertError { .. }
//             | DbError::UpdateError { .. }
//             | DbError::QueryExecuteError { .. } => Self::DbFail(db_error.to_string()),
//             // DbError::NoConnectionAvailable(_) => (),
//             // DbError::NotFound(_) => (),
//             _ => Self::Unknown,
//         }
//     }
// }
