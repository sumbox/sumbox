use axum::{http::StatusCode};

#[derive(thiserror::Error, Debug)]
pub enum WebError {
    // New error variant!
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Return a 401 for auth errors
            WebError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
