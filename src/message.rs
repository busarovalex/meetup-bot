use telegram_bot::{Integer, User, Chat, MessageType};
use telegram_bot::Message as TelegramMessage;

use ::chatroom::*;
use ::command::*;
use ::error::*;

use std::convert::TryFrom;

pub struct Message {
    pub user: User,
    pub chat: ChatRoom,
    pub date: Integer,
    pub text: String,
    pub command: Command
}

impl TryFrom<TelegramMessage> for Message {
    type Err = MeetupError;
    fn try_from(raw: TelegramMessage) -> Result<Self, MeetupError> {
        match (raw.chat, raw.msg) {
            (Chat::Group{id, title, is_supergroup}, MessageType::Text(text)) => {
                let command = Command::from(&text[..]);
                Ok(Message {
                    user: raw.from,
                    chat: ChatRoom {id: id, title: title, is_supergroup: is_supergroup },
                    date: raw.date,
                    text: text,
                    command: command
                })
            },
            (Chat::Group{..}, _) => Err(MeetupError::ChatIsNotGroup),
            (_, _) => Err(MeetupError::NotATextMessage)
        }
    }
}
