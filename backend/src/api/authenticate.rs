
use axum::{extract::Json, http::StatusCode, Router, routing::post};
use axum_extra::extract::cookie::{CookieJar, Cookie};

use super::super::types::{User,Claims};

pub fn create_route() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logoff", post(logout))
}

pub async fn login(Json(body): Json<User>, jar:CookieJar) -> Result<(CookieJar, String), (StatusCode, String)> {
    if body.is_valid() {
        if jar.get("sumboxlogin").is_none() {
            let token = Claims::encode(&body);
            Ok(( jar.add(Cookie::new("sumboxlogin", token)), String::from("OK")))
        } else {
            Err({
                (StatusCode::UNAUTHORIZED, "Already Logged In".to_string())
            })
        }
    }   else {
        Err({
            (StatusCode::UNAUTHORIZED, "Invalid Credentials".to_string())
        })
    }
}

pub async fn logout(jar:CookieJar) -> Result<(CookieJar, String), (StatusCode, String)> {
    if jar.get("sumboxlogin").is_some() {
        Ok((jar.remove(Cookie::named("sumboxlogin")), String::from("OK")))
    } else {
        Err({
            (StatusCode::UNAUTHORIZED, "Not Logged In".to_string())
        })
    }
}
