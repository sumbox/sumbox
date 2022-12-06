use axum::extract::Json;
use axum::{Extension, Router};
use axum::routing::post;
use common::AppError;
use hyper::StatusCode;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

type Database = Extension<std::sync::Arc<db::PrismaClient>>;

use crate::{types::{Account}, db::{self, account}};

pub fn create_route() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logoff", post(logoff))
}   

pub async fn register(_db: Database, Json(_input): Json<Account>) -> AppResult<StatusCode> {
    let _ = _db
        .account()
        .create(
            _input.email,
            hash_password(_input.password),
            vec![
                account::name::set(_input.name),
                account::role::set(_input.role),
            ],
        )
        .exec()
        .await?;

    Ok(StatusCode::OK)
}


pub async fn login(_db: Database, Json(_input): Json<Account>) -> AppJsonResult<Option<account::Data>> {
    let acc: Option<account::Data> = _db.account().find_first(vec![account::email::equals(_input.email), 
                                                                account::password::equals(hash_password(_input.password))]).exec().await?;
    Ok(Json::from(acc))
}

pub async fn logoff(_db: Database, Json(_input): Json<Account>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

fn hash_password(_password: String) -> String {
    
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    argon2.hash_password(_password.as_bytes(), &salt).unwrap().to_string()
}


#[cfg(test)]
mod tests {

    use super::*;
    // Add tests here
    #[tokio::test]
    async fn test_create() {
        let client = std::sync::Arc::new(db::new_client().await.unwrap());

        #[cfg(debug)]
        client._db_push(false).await.unwrap();

        let acc = Account {id: None, email: "aaa@bbb.ccc".to_string(), password: "password".to_string(), name: None, role: crate::db::Role::Account, profile: None, votes: None, comments: None };
        let res = register(axum::Extension(client), Json(acc)).await;
        
        assert!(res.is_ok());  
    }

    #[tokio::test]
    async fn test_login() {
        let client = std::sync::Arc::new(db::new_client().await.unwrap());

        #[cfg(debug)]
        client._db_push(false).await.unwrap();

        let acc = Account {id: None, email: "aaa@bbb.ccc".to_string(), password: "password".to_string(), name: None, role: crate::db::Role::Account, profile: None, votes: None, comments: None };
        let res = login(axum::Extension(client), Json(acc)).await;
        println!("{}",res.unwrap());
        assert!(res.is_ok());
    }    

}


