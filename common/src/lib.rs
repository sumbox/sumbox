use prisma_client_rust::{QueryError,};
use axum::{http::StatusCode};
use axum::response::{IntoResponse, Response};
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};

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
    use super::*;

    // Add tests here 
    // #[test]
    // fn it_works() { 
    //     assert_eq!(1,1);
    // }
}
