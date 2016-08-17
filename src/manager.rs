use ::meetup::*;

use std::collections::HashMap;

pub struct Manager {
    meetups: HashMap<String, Meetup>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            meetups: HashMap::new()
        }
    }
    
    pub fn create_or_find(&mut self, secret_phrase: &str) -> &mut Meetup {
        if self.meetups.contains_key(secret_phrase) {
            return match self.meetups.get_mut(secret_phrase) {
                Some(meetup) => meetup,
                None => unreachable!()
            }
        }  
        
        self.meetups.insert(secret_phrase.to_owned(), Meetup::new(secret_phrase.to_owned()));
        self.meetups.get_mut(secret_phrase).unwrap()
    }
    
    pub fn find(&mut self, secret_phrase: &str) -> Option<&mut Meetup> {
        self.meetups.get_mut(secret_phrase)
    }
}
