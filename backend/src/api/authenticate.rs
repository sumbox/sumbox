use axum::{
    headers::{
        authorization::{Basic},
        Authorization
    },
    routing::post,
    Router, TypedHeader,
};

use sha3::Digest;
use super::{
    super::types::Claims,
    ideas::{AppResult, Database},
};
use crate::db::user;
use anyhow::anyhow;
use common::AuthError;
use hyper::StatusCode;
use secrecy::{ExposeSecret, Secret};

pub fn create_route() -> Router {
    Router::new().route("/login", post(login))
    // .route("/logoff", post(logout))
}

async fn login(
    TypedHeader(headers): TypedHeader<Authorization<Basic>>,
    db: Database,
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

async fn validate_credentials(
    credentials: &Claims,
    db: &Database,
) -> Result<String, anyhow::Error> {
    let user_query = db
        .user()
        .find_first(vec![user::username::equals(String::from(
            &credentials.username,
        )), user::password::equals({
            format!("{:x}", sha3::Sha3_256::digest(
                credentials.password.expose_secret().as_bytes()
            ))
        })])
        .exec()
        .await?;
    match user_query {
        Some(data) => {
            Ok(data.id)
        }
        None => Err(anyhow!(AuthError::InvalidCredentials(String::from(
            "Invalid username"
        )))),
    }
}
