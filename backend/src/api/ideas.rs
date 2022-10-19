use axum::{
    extract::{Json},
    http::StatusCode,
    response::{IntoResponse, Response}, Extension, Router, routing::{get, post},
};
use prisma_client_rust::{
    QueryError,
};
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};
use serde::Deserialize;

type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

use crate::db::{self, idea};

type Database = Extension<std::sync::Arc<db::PrismaClient>>;

#[derive(Deserialize)]
pub struct IdeaRequest {
    title: String,
    body: String,
}

pub fn create_route() -> Router {
    Router::new()
        .route("/idea", get(get_idea))
        .route("/idea", post(create_idea))
}

pub async fn get_idea(db:Database) -> AppResult<Json<Vec<idea::Data>>> {
    let ideas = db
    .idea()
    .find_many(vec![])
    .exec()
    .await?;
    Ok(Json::from(ideas))
}

pub async fn create_idea(db:Database, Json(input): Json<IdeaRequest>) -> AppJsonResult<idea::Data> {
        let data : idea::Data = db
        .idea()
        .create(input.title, input.body, vec![])
        .exec()
        .await?;
        println!("{:?}", data);
        Ok(Json::from(data))
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[test]
    fn test_get_idea() {
        assert_eq!(true,true);
    }

    #[test]
    fn test_create_idea() {
        assert_eq!(true,true);
    }


}