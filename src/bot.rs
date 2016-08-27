use telegram_bot::{Api};
use telegram_bot::Message as TelegramMessage;

use ::manager::*;
use ::message::*;
use ::error::*;
use ::send::*;

use std::convert::TryFrom;

pub struct Bot<'r> {
    api: &'r Api,
    manager: Manager
}

impl<'r> Bot<'r> {
    pub fn new(api: &'r Api) -> Bot<'r> {
        Bot {
            api: api,
            manager: Manager::new()
        }
    }
    
    pub fn process_message(&mut self, msg: TelegramMessage) {
        match Message::try_from(msg) {
            Ok(message) => {
                if let Some(command) = message.command {
                    let mut chat_room = self.manager.create_or_find(message.chat_id);
                    let payload = command.execute(message.user, chat_room);
                    self.api.send((message.chat_id, payload));
                }
            },
            Err(error) => {
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
