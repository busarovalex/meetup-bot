use telegram_bot::*;

use super::vote::*;

use std::collections::HashMap;

pub struct Meetup {
    pub users: HashMap<Integer, (User, Vote)>,
    pub name: Option<String>
}

impl Meetup {
    pub fn new() -> Meetup {
        Meetup {
            users: HashMap::new(),
            name: None
        }
    }
    
    pub fn with_name(name: String) -> Meetup {
        Meetup {
            users: HashMap::new(),
            name: Some(name)
        }
    }
    
    pub fn vote(&mut self, user: User, vote: Vote) -> String {
        self.users.insert(user.id, (user, vote));
        vote.description()
    }
}
