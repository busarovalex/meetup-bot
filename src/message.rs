use telegram_bot::{Integer, User, Chat, MessageType};
use telegram_bot::Message as TelegramMessage;

use ::command::*;
use ::error::*;

use std::convert::TryFrom;

pub struct Message {
    pub user: User,
    pub chat_id: Integer,
    pub date: Integer,
    pub text: String,
    pub command: Option<Box<Command>>
}

impl TryFrom<TelegramMessage> for Message {
    type Err = MeetupError;
    fn try_from(raw: TelegramMessage) -> Result<Self, MeetupError> {
        let chat_id = raw.chat.id();
        match (raw.chat, raw.msg) {
            (Chat::Group{id, ..}, MessageType::Text(text)) => {
                let command = ::command::parse_command(&text[..]);
                Ok(Message {
                    user: raw.from,
                    chat_id: id,
                    date: raw.date,
                    text: text,
                    command: command
                })
            },
            (Chat::Private{..}, _) => Err(MeetupError::chat_is_not_group(ErrorDestination::new(raw.from, chat_id))),
            (Chat::Channel{..}, _) => Err(MeetupError::chat_is_not_group(ErrorDestination::new(raw.from, chat_id))),
            (_, _) => Err(MeetupError::not_a_text_message(ErrorDestination::new(raw.from, chat_id)))
        }
    }
}
