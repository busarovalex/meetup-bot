use telegram_bot::*;

use super::vote::*;

use std::collections::HashMap;

pub struct Meetup {
    pub users: HashMap<Integer, (User, Vote)>,
}

impl Meetup {
    pub fn new() -> Meetup {
        Meetup {
            users: HashMap::new()
        }
    }
    
    pub fn vote(&mut self, user: User, vote: Vote) -> String {
        self.users.insert(user.id, (user, vote));
        vote.description()
    }
}
