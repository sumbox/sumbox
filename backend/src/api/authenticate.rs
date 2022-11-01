use std::any;

// use anyhow::Context;
use axum::{ routing::post, Router, TypedHeader, headers::{Authorization, authorization::{Basic, Credentials}, Error}};

use hyper::StatusCode;
use secrecy::{Secret, ExposeSecret};
use common::AuthError;
use anyhow::anyhow;
use super::{super::types::{Claims}, ideas::{AppResult, Database}};
use crate::db::{user};

pub fn create_route() -> Router {
    Router::new()
        .route("/login", post(login))
        // .route("/logoff", post(logout))
}

async fn login(
    TypedHeader(headers): TypedHeader<Authorization<Basic>>,
    db: Database
) -> AppResult<()> {
    let credentials = auth(&headers).map_err(|e| {
        tracing::error!("Error authenticating: {}", e);
        StatusCode::UNAUTHORIZED
    });

    let s = validate_credentials(&credentials.unwrap(), &db).await?;
    println!("{}", s);

    Ok(())
}


fn auth(header: &Authorization<Basic>) -> Result<Claims, anyhow::Error> {
    Ok(Claims {
        username: header.username().to_owned(),
        password: Secret::new(header.password().to_owned()),
    })
}

async fn validate_credentials(credentials: &Claims, db: &Database) -> Result<String, anyhow::Error> {

    let user_query = db.user().find_first(vec![user::username::equals(String::from(&credentials.username))]).exec().await?;
    match user_query {
        Some(data) => {
            let password = data.password;
            let password = password;
            if &password == credentials.password.expose_secret() {
                Ok(data.id)
            } else {
                Err(anyhow!(AuthError::InvalidCredentials(String::from("Invalid credentials"))))
            }
        }
        None => Err(anyhow!(AuthError::InvalidCredentials(String::from("Invalid username")))),
    }

}