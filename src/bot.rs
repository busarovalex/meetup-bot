use telegram_bot::*;

use ::manager::*;
use ::command::*;

pub struct Bot {
    manager: Manager
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            manager: Manager::new()
        }
    }
    
    pub fn process_message(&mut self, api: &Api, msg: Message) {
        if let Some(command) = Self::extract_command(&msg) {
            self.process_command(api, command, msg.from, msg.chat);
        }
    }
    
    pub fn process_command(&mut self, api: &Api, command: Command, user: User, chat: Chat) {
        match command {
            Command::JoinMeetup(secret_phrase) => self.join_meetup(api, secret_phrase, user, chat),
            Command::LeaveMeetup(Some(secret_phrase)) => self.leave_meetup(api, secret_phrase, user, chat),
            _ => {}
        }
    }
    
    fn join_meetup(&mut self, api: &Api, secret_phrase: String, user: User, chat: Chat) {
        let meetup = self.manager.create_or_find(&secret_phrase[..]);
        meetup.add_user(user);
        let response = format!("You joined {} meetup!", &secret_phrase);
        api.send_message(chat.id(), response, None, None, None, None).unwrap();
    }
    
    fn leave_meetup(&mut self, api: &Api, secret_phrase: String, user: User, chat: Chat) {
        if let Some(ref mut meetup) = self.manager.find(&secret_phrase[..]) {
            meetup.remove_user(&user);
            let response = format!("You left {} meetup!", &secret_phrase);
            api.send_message(chat.id(), response, None, None, None, None).unwrap();
        } else {
            let response = format!("You do not belong to {} meetup!", &secret_phrase);
            api.send_message(chat.id(), response, None, None, None, None).unwrap();
        }
    }
    
    fn extract_command(msg: &Message) -> Option<Command> {
        if let MessageType::Text(ref payload) = msg.msg {
            return Some(Command::from(&payload[..]));
        }
        None
    }
}
