use telegram_bot::{Integer};

use ::meetup::*;

use std::collections::HashMap;

pub struct Manager {
    meetups: HashMap<Integer, Meetup>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            meetups: HashMap::new()
        }
    }
    
    pub fn create_or_find(&mut self, chat_id: Integer) -> &mut Meetup {
        if self.meetups.contains_key(&chat_id) {
            return match self.meetups.get_mut(&chat_id) {
                Some(meetup) => meetup,
                None => unreachable!()
            }
        }  
        
        self.meetups.insert(chat_id, Meetup::new());
        self.meetups.get_mut(&chat_id).unwrap()
    }
}
