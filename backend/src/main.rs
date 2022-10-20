use axum::{
    extract::Extension, 
    Router, 
    body::{Body, Bytes}, 
    middleware::{self, Next}, 
    http::{Request,StatusCode}, 
    response::{IntoResponse, Response}
};

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "example_print_request_response=debug,tower_http=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();



    let app = Router::new()
        .nest("/api", api::ideas::create_route())
        .nest("/auth",api::authenticate::create_route()) 
        .layer(Extension(prisma_client))
        .layer(middleware::from_fn(print_request_response));

    axum::Server::bind(&config.server_address)
    .serve(app.into_make_service())
    .await
    .unwrap();
}


async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}


