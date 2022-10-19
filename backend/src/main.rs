use axum::{extract::Extension, Router};
use std::sync::Arc;

mod db;
mod config;
mod types;
mod api;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let prisma_client = Arc::new(db::new_client().await.unwrap());
    #[cfg(debug)]
    prisma_client._db_push(false).await.unwrap();
    
    let app = Router::new()
        .nest("/api", api::ideas::create_route())
        .nest("/auth",api::authenticate::create_route()) 
        .layer(Extension(prisma_client));


    axum::Server::bind(&config.server_address)
    .serve(app.into_make_service())
    .await
    .unwrap();
}


