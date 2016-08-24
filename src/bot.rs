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
    
//    fn process_command(&mut self, command: Command, user: User, chat: Chat) {
//        match command {
//            Command::JoinMeetup(secret_phrase) => self.join_meetup(secret_phrase, user, chat),
//            Command::LeaveMeetup(Some(secret_phrase)) => self.leave_meetup(secret_phrase, user, chat),
//            _ => {}
//        }
//    }
    
//    fn join_meetup(&mut self, secret_phrase: String, user: User, chat: Chat) {
//        let meetup = self.manager.create_or_find(&secret_phrase[..]);
//        meetup.add_user(user);
//        let response = format!("You joined {} meetup!", &secret_phrase);
//        self.api.send_message(chat.id(), response, None, None, None, None).unwrap();
//    }
//    
//    fn leave_meetup(&mut self, secret_phrase: String, user: User, chat: Chat) {
//        if let Some(ref mut meetup) = self.manager.find(&secret_phrase[..]) {
//            meetup.remove_user(&user);
//            let response = format!("You left {} meetup!", &secret_phrase);
//            self.api.send_message(chat.id(), response, None, None, None, None).unwrap();
//        } else {
//            let response = format!("You do not belong to {} meetup!", &secret_phrase);
//            self.api.send_message(chat.id(), response, None, None, None, None).unwrap();
//        }
//    }
    
//    fn extract_command(msg: &TelegramMessage) -> Option<Command> {
//        if let MessageType::Text(ref payload) = msg.msg {
//            return Some(Command::from(&payload[..]));
//        }
//        None
//    }
}
