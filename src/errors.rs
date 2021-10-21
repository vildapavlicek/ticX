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
}

// this shows error because it cannot indetify std::fmt::Display being derived
impl actix_web::error::ResponseError for TicxError {}
