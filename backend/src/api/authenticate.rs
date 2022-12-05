use axum::extract::Json;
use axum::{Extension, Router};

use axum::routing::get;
use axum::routing::post;
use common::AppError;
use hyper::StatusCode;
use serde::Deserialize;

type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

type Database = Extension<std::sync::Arc<db::PrismaClient>>;
use crate::db::Role;
use crate::db::{self, account};
use crate::types::{Account, Comment, Profile, Vote};

pub fn create_route() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logoff", post(logoff))
}

pub async fn register(db: Database, Json(input): Json<Account>) -> AppResult<StatusCode> {
    let account = db
        .account()
        .create(
            input.email,
            input.password,
            vec![
                account::name::set(input.name),
                account::role::set(input.role),
            ],
        )
        .exec()
        .await?;

    Ok(StatusCode::OK)
}

pub async fn login(db: Database, Json(input): Json<Account>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

pub async fn logoff(db: Database, Json(input): Json<Account>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
