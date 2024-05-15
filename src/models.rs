use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}
impl User {
    pub fn new(email: String, password: String) -> User {
        User { email, password }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
impl NewUser {
    pub fn new(name: String, email: String, password: String) -> NewUser {
        NewUser {
            name,
            email,
            password,
        }
    }
}
