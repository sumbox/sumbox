use axum::{Router, Extension};
use axum::extract::{Json};

use serde::{Deserialize};
use axum::routing::get;
use axum::routing::post;

use common::{AppError};

type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

type Database = Extension<std::sync::Arc<db::PrismaClient>>;
use crate::db::{self, idea};

#[derive(Deserialize)]
pub struct IdeaRequest { field1: String, field2: String }

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
        .create(input.field1, input.field2, vec![])
        .exec()
        .await?;
        println!("{:?}", data);
        Ok(Json::from(data))
}


#[cfg(test)]
mod tests {
    // use super::*;

    // Add tests here 
    // #[test]
    // fn it_works() { 
    //     assert_eq!(1,1);
    // }
}