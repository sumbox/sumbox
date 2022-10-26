use serde::Deserialize;
use std::env::var;

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn is_valid(&self) -> bool {
        let email = var("AUTH_EMAIL").expect("AUTH_EMAIL must be set");
        let password = var("AUTH_PASSWORD").expect("AUTH_PASSWORD must be set");

        self.email == email && self.password == password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test Valid
    #[test]
    fn test_is_valid() {
        let eml = var("AUTH_EMAIL").expect("AUTH_EMAIL must be set");
        let pwd = var("AUTH_PASSWORD").expect("AUTH_PASSWORD must be set");
        let v: User = User {
            email: eml,
            password: pwd,
        };
        assert!(v.is_valid())
    }

    // Test Invalid
    #[test]
    fn test_is_invalid() {
        let eml = var("AUTH_EMAIL").expect("AUTH_EMAIL must be set");
        let pwd = String::from("aaa");
        let v: User = User {
            email: eml,
            password: pwd,
        };
        assert!(!v.is_valid())
    }
}
