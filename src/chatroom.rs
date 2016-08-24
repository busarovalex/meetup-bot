use telegram_bot::*;

use ::meetup::MeetupList;

pub struct ChatRoom {
    id: Integer,
    meetup_list: MeetupList
}

impl ChatRoom {
    pub fn new(id: Integer) -> ChatRoom {
        ChatRoom {
            id: id,
            meetup_list: MeetupList::new()
        }
    }
    
    pub fn meetup_list(&mut self) -> &mut MeetupList {
        &mut self.meetup_list
    }
}
