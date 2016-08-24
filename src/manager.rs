use telegram_bot::{Integer};

use ::chatroom::*;

use std::collections::HashMap;

pub struct Manager {
    meetups: HashMap<Integer, ChatRoom>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            meetups: HashMap::new()
        }
    }
    
    pub fn create_or_find(&mut self, chat_id: Integer) -> &mut ChatRoom {
        if self.meetups.contains_key(&chat_id) {
            return match self.meetups.get_mut(&chat_id) {
                Some(meetup) => meetup,
                None => unreachable!()
            }
        }  
        
        self.meetups.insert(chat_id, ChatRoom::new(chat_id));
        self.meetups.get_mut(&chat_id).unwrap()
    }
}
