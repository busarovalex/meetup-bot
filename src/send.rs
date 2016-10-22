use telegram_bot::{Api, Integer};
use model::*;
use chat_room::*;

use ::error::*;

pub trait SendMessage {
    fn send<T: Payload>(&self, payload: T);
}

pub trait Payload {
    fn chat(&self) -> Integer;
    fn text(&self) -> String;
}

impl SendMessage for Api {
    fn send<T: Payload>(&self, payload: T) {
        self.send_message(payload.chat(), payload.text(), None, None, None, None).unwrap();
    }
}

impl Payload for MeetupError {
    fn chat(&self) -> Integer {
        self.destination.chat_id
    }
    
    fn text(&self) -> String {
        self.error_type.description()
    }
}

impl Payload for (Integer, String) {
    fn chat(&self) -> Integer {
        self.0
    }
    
    fn text(&self) -> String {
        self.1.clone()
    }
}

impl Payload for (ChatRoomId, OutgoingMessage) {
    fn chat(&self) -> Integer {
        *self.0
    }
    
    fn text(&self) -> String {
        self.1.text.clone()
    }
}
