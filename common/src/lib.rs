use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};
use prisma_client_rust::QueryError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    details: String
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub enum AppError {
    PrismaError(QueryError),
    NotFound
}


impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all differents errors from our app in one place
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