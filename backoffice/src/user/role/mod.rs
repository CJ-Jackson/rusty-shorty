pub mod visitor_only;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    ROOT,
    USER,
    VISITOR,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "root" => Self::ROOT,
            "user" => Self::USER,
            "visitor" => Self::VISITOR,
            _ => panic!("Invalid role"),
        }
    }
}

impl From<Role> for String {
    fn from(r: Role) -> Self {
        match r {
            Role::ROOT => "root".to_string(),
            Role::USER => "user".to_string(),
            Role::VISITOR => "visitor".to_string(),
        }
    }
}

impl Role {
    pub fn level(&self) -> u8 {
        match self {
            Self::ROOT => 2,
            Self::USER => 1,
            Self::VISITOR => 0,
        }
    }

    pub fn all_roles() -> Vec<Self> {
        vec![Self::ROOT, Self::USER, Self::VISITOR]
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.level().partial_cmp(&other.level())
    }
}
