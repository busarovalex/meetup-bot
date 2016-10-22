use super::Id;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>
}

pub type UserId = Id<i64, User>;
