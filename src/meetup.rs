use telegram_bot::*;

use std::collections::HashMap;

pub struct Meetup {
    pub users: HashMap<Integer, User>,
    pub secret_phrase: String
}

impl Meetup {
    pub fn new(secret_phrase: String) -> Meetup {
        Meetup {
            users: HashMap::new(),
            secret_phrase: secret_phrase
        }
    }
    
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    
    pub fn remove_user(&mut self, user: &User) {
        self.users.remove(&user.id);
    }
}
