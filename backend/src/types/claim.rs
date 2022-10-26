use std::env::var;

use super::User;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn encode(user: &User) -> String {
        let claims = Claims {
            sub: user.email.clone(),
            exp: 10000000000,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(
                var("AUTH_SECRET")
                    .expect("AUTH_SECRET should be set")
                    .as_ref(),
            ),
        )
        .expect("Failed to encode cookie");

        token
    }
}
