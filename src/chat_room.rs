use typemap::*;

use model::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct ChatRoom {
    id: ChatRoomId,
    pub users: HashMap<UserId, User>,
    pub type_map: DebugMap
}

pub type ChatRoomId = Id<i64, ChatRoom>;


impl ChatRoom {
    pub fn new(id: ChatRoomId) -> ChatRoom {
        ChatRoom {
            id: id,
            users: HashMap::new(),
            type_map: TypeMap::custom()
        }
    }

    pub fn id(&self) -> ChatRoomId {
        self.id
    }

    pub fn add_user_if_not_present(&mut self, user: &User) {
        if !self.users.contains_key(&user.id) {
            self.users.insert(user.id, user.clone());
        } 
    }
}
