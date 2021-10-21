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
}

// this shows error because it cannot indetify std::fmt::Display being derived
impl actix_web::error::ResponseError for TicxError {
    fn status_code(&self) -> StatusCode {
        match self {
            TicxError::MissingAuthHeader => StatusCode::BAD_REQUEST,
            TicxError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
