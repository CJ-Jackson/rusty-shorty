use crate::user::role::Role;

pub struct ListUser {
    pub id: i64,
    pub username: String,
    pub role: Role,
}

pub struct FetchUser {
    pub username: String,
    pub role: Role,
}
