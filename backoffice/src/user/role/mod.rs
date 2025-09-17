#[derive(Debug, Clone, PartialEq)]
pub enum Roles {
    ROOT,
    USER,
    VISITOR,
}

impl From<&str> for Roles {
    fn from(s: &str) -> Self {
        match s {
            "root" => Self::ROOT,
            "user" => Self::USER,
            "visitor" => Self::VISITOR,
            _ => panic!("Invalid role"),
        }
    }
}

impl From<Roles> for String {
    fn from(r: Roles) -> Self {
        match r {
            Roles::ROOT => "root".to_string(),
            Roles::USER => "user".to_string(),
            Roles::VISITOR => "visitor".to_string(),
        }
    }
}

impl Roles {
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

impl PartialOrd for Roles {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.level().partial_cmp(&other.level())
    }
}
