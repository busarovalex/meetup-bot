use model::*;
use chat_room::*;

pub trait Handler {
    fn handle(&self, message: &IncomingMessage, chat_room: &mut ChatRoom) -> Option<OutgoingMessage>;
    fn name(&self) -> Option<String> { None }
}
