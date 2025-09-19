use crate::user::role::Role;

pub struct ListUser {
    id: i64,
    username: String,
    role: Role,
}

pub struct FetchUser {
    username: String,
    role: Role,
}
