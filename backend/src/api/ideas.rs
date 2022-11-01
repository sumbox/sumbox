use axum::extract::Json;
use axum::{Extension, Router};

use axum::routing::get;
use axum::routing::post;
use serde::Deserialize;

use common::AppError;

pub type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

type Database = Extension<std::sync::Arc<db::PrismaClient>>;
use crate::db::{self, idea};

#[derive(Deserialize)]
pub struct IdeaRequest {
    field1: String,
    field2: String,
}

pub fn create_route() -> Router {
    Router::new()
        .route("/idea", get(read_idea))
        .route("/idea", post(create_update_idea))
}

pub async fn read_idea(db: Database) -> AppResult<Json<Vec<idea::Data>>> {
    let ideas: Vec<idea::Data> = db.idea().find_many(vec![]).exec().await?;
    Ok(Json::from(ideas))
}

// POST/PUT
pub async fn create_update_idea(
    db: Database,
    Json(input): Json<IdeaRequest>,
) -> AppJsonResult<idea::Data> {
    let data: idea::Data = db
        .idea()
        .create(input.field1, input.field2, vec![])
        .exec()
        .await?;
    println!("{:?}", data);
    Ok(Json::from(data))
}

#[cfg(test)]
mod tests {
    use super::*;
    // Add tests here
    #[tokio::test]
    async fn it_works() {
        let client = std::sync::Arc::new(db::new_client().await.unwrap());

        #[cfg(debug)]
        client._db_push(false).await.unwrap();

        let ideas: Vec<idea::Data> = client.idea().find_many(vec![]).exec().await.unwrap();
        print!("Ideas {:?}", ideas);
        assert_eq!(ideas.len(), 0);
    }
}
