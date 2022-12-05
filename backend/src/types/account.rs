use crate::db::Role;
use crate::types::{Comment, Profile, Vote};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub role: Role,
    pub profile: Option<Profile>,
    pub votes: Option<Vec<Vote>>,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);
