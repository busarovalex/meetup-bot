use chat_room::*;

use std::collections::HashMap;

pub struct Manager {
    rooms: HashMap<ChatRoomId, ChatRoom>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            rooms: HashMap::new()
        }
    }
    
    pub fn create_or_find(&mut self, chat_room_id: ChatRoomId) -> &mut ChatRoom {
        if self.rooms.contains_key(&chat_room_id) {
            return match self.rooms.get_mut(&chat_room_id) {
                Some(meetup) => meetup,
                None => unreachable!()
            }
        }  
        
        self.rooms.insert(chat_room_id, ChatRoom::new(chat_room_id));
        self.rooms.get_mut(&chat_room_id).unwrap()
    }
}
