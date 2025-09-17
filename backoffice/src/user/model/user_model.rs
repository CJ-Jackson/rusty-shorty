use crate::user::role::Role;

#[derive(Debug)]
pub struct UserIdContext {
    pub id: i64,
    pub username: String,
    pub role: Role,
}
