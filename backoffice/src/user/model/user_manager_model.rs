use crate::user::role::Role;
use serde::Serialize;

#[derive(Serialize)]
pub struct ListUser {
    pub id: i64,
    pub username: String,
    pub role: Role,
}

pub struct FetchUser {
    pub username: String,
    pub role: Role,
}

pub struct FetchPassword {
    pub password: Box<[u8]>,
}
