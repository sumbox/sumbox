
use std::env::var;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn is_valid(&self) -> bool {
        let email = var("AUTH_EMAIL").expect("AUTH_EMAIL must be set");
        let password = var("AUTH_PASSWORD").expect("AUTH_PASSWORD must be set");

        if self.email==email && self.password==password {
            return true
        } else {
            return false
        }
    }
}