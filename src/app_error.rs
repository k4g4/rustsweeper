use http::status::StatusCode;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Invalid Difficulty")]
    InvalidDifficulty,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidDifficulty => StatusCode::BAD_REQUEST,
        }
    }
}
