use anyhow::Context;
use axum::{ routing::post, Router, TypedHeader, headers::{Authorization, authorization::{Basic, Credentials}}};

use hyper::StatusCode;
use secrecy::Secret;

use crate::types::WebError;

use super::{super::types::{Claims}, ideas::AppResult};

pub fn create_route() -> Router {
    Router::new()
        .route("/login", post(login))
        // .route("/logoff", post(logout))
}

async fn login(
    TypedHeader(headers): TypedHeader<Authorization<Basic>>,
) -> AppResult<()> {
    let _credentials = auth(&headers);
    println!("Credentials {:?}", _credentials);
    Ok(())
}


fn auth(header: &Authorization<Basic>) -> Result<Claims, anyhow::Error> {

    Ok(Claims {
        username: header.username().to_owned(),
        password: Secret::new(header.password().to_owned()),
    })
}