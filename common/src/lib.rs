use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};
use prisma_client_rust::QueryError;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials(String),
    MissingAuthorizationHeader(String),
    InvalidAuthorizationHeader(String),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials(msg) => write!(f, "Invalid credentials: {}", msg),
            AuthError::MissingAuthorizationHeader(msg) => write!(f, "Missing authorization header: {}", msg),
            AuthError::InvalidAuthorizationHeader(msg) => write!(f, "Invalid authorization header: {}", msg),
        }
    }
}

pub enum AppError {
    PrismaError(QueryError),
    NotFound,
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}


impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        match error {
            // AuthError
            e if e.is::<AuthError>() => {
                println!("AuthError: {}", e);
                AppError::NotFound
            }
            _ => {
                AppError::NotFound
            }
        }
    }
}


// This centralizes all different's errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // Add tests here
    // #[test]
    // fn it_works() {
    //     assert_eq!(1,1);
    // }
}
