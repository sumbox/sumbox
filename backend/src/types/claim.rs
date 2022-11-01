use secrecy::Secret;

#[derive(Debug)]
pub struct Claims {
    pub username : String,
    pub password: Secret<String>,
}