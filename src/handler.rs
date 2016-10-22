use model::*;
use chat_room::*;

pub trait Handler {
    fn handle(&self, message: &IncomingMessage, chat_room: &ChatRoom) -> Option<OutgoingMessage>;
}
