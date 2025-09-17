pub mod user_role_check;
pub mod visitor_only;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Root,
    User,
    Visitor,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "root" => Self::Root,
            "user" => Self::User,
            "visitor" => Self::Visitor,
            _ => panic!("Invalid role"),
        }
    }
}

impl From<Role> for String {
    fn from(r: Role) -> Self {
        match r {
            Role::Root => "root".to_string(),
            Role::User => "user".to_string(),
            Role::Visitor => "visitor".to_string(),
        }
    }
}

impl Role {
    pub fn level(&self) -> u8 {
        match self {
            Self::Root => 2,
            Self::User => 1,
            Self::Visitor => 0,
        }
    }

    pub fn all_roles() -> Vec<Self> {
        vec![Self::Root, Self::User, Self::Visitor]
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.level().partial_cmp(&other.level())
    }
}
