use telegram_bot::*;

pub struct ChatRoom {
    pub id: Integer,
    pub title: String,
    pub is_supergroup: bool,
}
