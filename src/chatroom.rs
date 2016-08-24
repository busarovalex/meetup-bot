use telegram_bot::*;

use ::meetup::Meetup;

pub struct ChatRoom {
    id: Integer,
    meetup: Meetup
}

impl ChatRoom {
    pub fn new(id: Integer) -> ChatRoom {
        ChatRoom {
            id: id,
            meetup: Meetup::new()
        }
    }
    
    pub fn meetup(&mut self) -> &mut Meetup {
        &mut self.meetup
    }
}
