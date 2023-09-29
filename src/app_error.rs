use http::status::StatusCode;
use leptos_router::ParamsError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Error reading new game settings: {0}")]
    ParamsError(#[from] ParamsError),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ParamsError(_) => StatusCode::BAD_REQUEST,
        }
    }
}
