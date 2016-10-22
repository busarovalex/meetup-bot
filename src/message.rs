use telegram_bot::{Integer, Chat, MessageType};
use telegram_bot::Message as TelegramMessage;
use telegram_bot::User as TelegramUser;

use error::*;
use model::*;

use std::convert::TryFrom;


#[derive(Debug)]
pub struct Message {
    pub user: User,
    pub chat_id: Integer,
    pub date: Integer,
    pub text: String
}

impl Into<IncomingMessage> for Message {
    fn into(self) -> IncomingMessage {
        IncomingMessage {
            text: self.text,
            time: ::model::datetime(self.date),
            from: self.user
        }
    }
}

pub fn telegram_user_to_user(telegram_user: TelegramUser) -> User {
    User {
        id: Id::new(telegram_user.id),
        first_name: telegram_user.first_name,
        last_name: telegram_user.last_name,
        username: telegram_user.username
    }
}

impl TryFrom<TelegramMessage> for Message {
    type Err = MeetupError;
    fn try_from(raw: TelegramMessage) -> Result<Self, MeetupError> {
        let chat_id = raw.chat.id();
        match (raw.chat, raw.msg) {
            (Chat::Group{id, ..}, MessageType::Text(text)) => {
                Ok(Message {
                    user: telegram_user_to_user(raw.from),
                    chat_id: id,
                    date: raw.date,
                    text: text
                })
            },
            (Chat::Private{..}, _) => Err(MeetupError::chat_is_not_group(ErrorDestination::new(raw.from, chat_id))),
            (Chat::Channel{..}, _) => Err(MeetupError::chat_is_not_group(ErrorDestination::new(raw.from, chat_id))),
            (_, _) => Err(MeetupError::not_a_text_message(ErrorDestination::new(raw.from, chat_id)))
        }
    }
}
