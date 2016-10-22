use telegram_bot::{Api};
use telegram_bot::Message as TelegramMessage;

use manager::*;
use message::*;
use error::*;
use send::*;
use handler::*;
use model::*;

use std::convert::TryFrom;

pub struct Bot<'r> {
    api: &'r Api,
    manager: Manager,
    handlers: Vec<Box<Handler>>
}

impl<'r> Bot<'r> {
    pub fn new(api: &'r Api) -> Bot<'r> {
        let handlers = vec![];
        Bot {
            api: api,
            manager: Manager::new(),
            handlers: handlers
        }
    }
    
    pub fn process_message(&mut self, msg: TelegramMessage) {
        match Message::try_from(msg) {
            Ok(message) => {
                debug!("Message: {:?}", &message);
                let mut chat_room = self.manager.create_or_find(Id::new(message.chat_id));
                chat_room.add_user_if_not_present(&message.user);
                let incoming_message = message.into();
                for handler in &self.handlers {
                    if let Some(outgoing_message) = handler.handle(&incoming_message, &chat_room) {
                        self.api.send((chat_room.id(), outgoing_message));
                    }
                }
            },
            Err(error) => {
                debug!("Failed convert telegram message to bot message: {:?}", &error);
                self.send_error_message(error);
            }
        }
    }
    
    fn send_error_message(&mut self, error: MeetupError) {
        if error.error_type.is_response_needed() {
            self.api.send(error);
        }
    }
}
